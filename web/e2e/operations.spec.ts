// What the HTTP API does, an assistant can do over MCP with the same sign-in: find an operation, see what it takes and
// call it, and no more than the token behind it may.
import { request, type APIRequestContext } from '@playwright/test'
import { expect, test, unique } from './support/test'

interface Called {
  result?: { structuredContent?: Record<string, unknown>; content: { text: string }[]; isError: boolean; tools?: { name: string }[] }
  error?: { message: string }
}

async function mcp(baseURL: string, token: string) {
  const ctx = await request.newContext({ baseURL, extraHTTPHeaders: { Authorization: `Bearer ${token}` } })
  let id = 0
  const rpc = async (method: string, params: object = {}): Promise<Called> =>
    (await (await ctx.post('/mcp', { data: { jsonrpc: '2.0', id: ++id, method, params } })).json()) as Called
  const call = async (name: string, args: object) => (await rpc('tools/call', { name, arguments: args })).result!
  return { ctx, rpc, call }
}

async function token(api: APIRequestContext, role: 'view' | 'edit'): Promise<{ id: number; token: string }> {
  const made = await api.post('/api/admin/tokens', { data: { name: `E2E ops ${role} ${unique()}`, role } })
  expect(made.ok()).toBe(true)
  return (await made.json()) as { id: number; token: string }
}

test.describe('operations over MCP', () => {
  test('an assistant finds, describes and calls the API with its token, and is recorded as MCP', async ({ api, baseURL, seed }) => {
    const edit = await token(api, 'edit')
    const { ctx, rpc, call } = await mcp(baseURL!, edit.token)
    try {
      const tools = (await rpc('tools/list')).result!.tools!.map((t) => t.name)
      expect(tools).toEqual(expect.arrayContaining(['find_operations', 'describe_operation', 'read_operation', 'run_operation']))

      // Found by what it does, in words.
      const found = await call('find_operations', { words: 'tag parents' })
      expect(found.isError).toBe(false)
      const ops = (found.structuredContent!.operations as { name: string }[]).map((o) => o.name)
      expect(ops[0]).toBe('set_tag_parents')
      // Or by group, from the list of groups.
      const groups = (await call('find_operations', {})).structuredContent!.groups as { group: string }[]
      expect(groups.map((g) => g.group)).toContain('tags')
      // What's for people to do themselves, and what the token can't, isn't offered.
      const everything = await Promise.all(groups.map((g) => call('find_operations', { group: g.group })))
      const names = everything.flatMap((r) => (r.structuredContent!.operations as { name: string }[]).map((o) => o.name))
      expect(names).toContain('merge_tags')
      expect(names).not.toContain('create_token')
      expect(names).not.toContain('change_settings')

      const described = (await call('describe_operation', { name: 'set_tag_parents' })).structuredContent!
      expect(described.call_with).toBe('run_operation')
      expect(JSON.stringify(described.body)).toContain('parents')

      // Put one tag under another, then read the tree back.
      const [child, parent] = [`e2eopschild${unique()}`, `e2eopsparent${unique()}`]
      const set = await call('run_operation', { name: 'set_tag_parents', body: { tag: child, parents: [parent] } })
      expect(set.isError, set.content[0].text).toBe(false)
      const tree = await call('read_operation', { name: 'get_tag_tree' })
      expect(tree.structuredContent!.result).toContainEqual({ tag: child, parent })
      // A read can't change, a run can't only look.
      expect((await call('read_operation', { name: 'set_tag_parents', body: { tag: child, parents: [] } })).isError).toBe(true)
      expect((await call('run_operation', { name: 'get_tag_tree' })).isError).toBe(true)

      // What it changes on an asset is in the asset's history as done over MCP.
      const asset = Object.values(seed.assets)[0]
      const description = `Described over MCP ${unique()}`
      const changed = await call('run_operation', { name: 'change_asset', path: { id: asset }, body: { description } })
      expect(changed.isError, changed.content[0].text).toBe(false)
      const history = (await call('read_operation', { name: 'get_asset_history', path: { id: asset } })).structuredContent!
        .result as { field: string; new: string; source: string }[]
      expect(history.find((h) => h.field === 'description' && h.new === description)?.source).toBe('mcp')

      // A file comes back as where to get it.
      const file = await call('read_operation', { name: 'get_asset_file', path: { id: asset } })
      expect(file.isError, file.content[0].text).toBe(false)
      expect(String(file.structuredContent!.url)).toContain(`/api/assets/${asset}/file`)
      // An operation that isn't there says where to look.
      const missing = await call('describe_operation', { name: 'fly_to_the_moon' })
      expect(missing.isError).toBe(true)
      expect(missing.content[0].text).toContain('find_operations')
    } finally {
      await ctx.dispose()
      await api.delete(`/api/admin/tokens/${edit.id}`)
    }
  })

  test('a token that can only look can read but not run', async ({ api, baseURL }) => {
    const view = await token(api, 'view')
    const { ctx, rpc, call } = await mcp(baseURL!, view.token)
    try {
      const tools = (await rpc('tools/list')).result!.tools!.map((t) => t.name)
      expect(tools).toContain('read_operation')
      expect(tools).not.toContain('run_operation')
      expect((await call('read_operation', { name: 'get_tag_tree' })).isError).toBe(false)
      const refused = await call('run_operation', { name: 'set_tag_parents', body: { tag: 'e2enope', parents: ['e2enoway'] } })
      expect(refused.isError).toBe(true)
      const found = (await call('find_operations', { words: 'tag parents' })).structuredContent!.operations as { name: string }[]
      expect(found.map((o) => o.name)).not.toContain('set_tag_parents')
      expect((await call('describe_operation', { name: 'merge_tags' })).isError).toBe(true)
    } finally {
      await ctx.dispose()
      await api.delete(`/api/admin/tokens/${view.id}`)
    }
  })
})
