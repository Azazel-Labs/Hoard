// Markdown (READMEs, licenses, notes that come with packs) drawn as a document. Raw HTML in the file is shown as
// text, not run, and markdown-it refuses javascript: and similar links, so a pack's README can't script the page.
import MarkdownIt from 'markdown-it'
import './markdown.css'

const md = new MarkdownIt({ html: false, linkify: true, typographer: true })

// Links leave Hoard in a new tab, and tell the other site nothing about where they came from.
md.renderer.rules.link_open = (tokens, i, options, _env, self) => {
  tokens[i].attrSet('target', '_blank')
  tokens[i].attrSet('rel', 'noopener noreferrer')
  return self.renderToken(tokens, i, options)
}

// Pictures on the web load (lazily, without a referrer); a picture beside the README in the pack has no address
// here, so it shows as its caption instead of a broken image.
md.renderer.rules.image = (tokens, i, _options, _env, self) => {
  const t = tokens[i]
  const src = t.attrGet('src') ?? ''
  const alt = self.renderInlineAsText(t.children ?? [], _options, _env)
  if (!/^https?:\/\//i.test(src)) return alt ? `<span class="md-img-alt">${md.utils.escapeHtml(alt)}</span>` : ''
  t.attrSet('alt', alt)
  t.attrSet('loading', 'lazy')
  t.attrSet('referrerpolicy', 'no-referrer')
  return self.renderToken(tokens, i, _options)
}

/** The file as HTML, safe to put in the page. */
export function renderMarkdown(text: string): string {
  return md.render(text)
}
