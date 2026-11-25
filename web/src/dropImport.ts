import { ref, type Ref } from 'vue'
import UploadQueue from './components/UploadQueue.vue'
import { filesFromDrop, groupsFromPicked, type Group } from './drop'
import { looksLikeFolderOfPacks, splitPerSubfolder } from './uploader'

// Something dragged from the page itself (a picture, a tile) isn't files to add, though a browser offers a picture as one.
let draggingOwn = false
document.addEventListener('dragstart', () => (draggingOwn = true))
document.addEventListener('dragend', () => (draggingOwn = false))

/** Files and folders dropped on the page or picked, handed to the import `queue`. `mayImport` says whether the person
 *  may add to the library. */
export function useDropImport(queue: Ref<InstanceType<typeof UploadQueue> | null>, mayImport: () => boolean) {
  /** A dropped/picked folder that contains several packs: ask how to import it. */
  const pendingSplit = ref<{ group: Group; parts: number } | null>(null)
  const dragDepth = ref(0)
  /** How many files or folders are being dragged over the page (0 when the browser won't say). */
  const dragCount = ref(0)

  /** Queue groups for import; a lone folder full of packs gets a choice first. */
  function importGroups(groups: Group[]) {
    if (!mayImport()) return
    if (groups.length === 1) {
      const parts = looksLikeFolderOfPacks(groups[0].files)
      if (parts) {
        pendingSplit.value = { group: groups[0], parts }
        return
      }
    }
    queue.value?.enqueue(groups)
  }

  function resolveSplit(split: boolean) {
    const p = pendingSplit.value
    pendingSplit.value = null
    if (!p) return
    const { name, files, handle } = p.group
    // Each part keeps the folder it came from, so a reload can read it again and pick its part back out.
    queue.value?.enqueue(split ? splitPerSubfolder(name, files).map((g) => ({ ...g, handle, root: name })) : [p.group])
  }

  function onDragEnter(e: DragEvent) {
    if (draggingOwn || !mayImport() || !e.dataTransfer?.types.includes('Files')) return
    if (!dragDepth.value) dragCount.value = e.dataTransfer.items?.length ?? 0
    dragDepth.value++
  }
  function onDragLeave() {
    dragDepth.value = Math.max(0, dragDepth.value - 1)
  }
  async function onDrop(e: DragEvent) {
    dragDepth.value = 0
    const own = draggingOwn
    draggingOwn = false
    if (!e.dataTransfer || own) return
    const groups = await filesFromDrop(e.dataTransfer)
    if (groups.length) importGroups(groups)
  }
  /** Files or a folder picked (a folder's files have a webkitRelativePath of "Folder/sub/file.png"). */
  function onPick(e: Event) {
    const input = e.target as HTMLInputElement
    const files = [...(input.files ?? [])]
    input.value = ''
    if (files.length) importGroups(groupsFromPicked(files))
  }

  return { pendingSplit, dragDepth, dragCount, resolveSplit, onDragEnter, onDragLeave, onDrop, onPick }
}
