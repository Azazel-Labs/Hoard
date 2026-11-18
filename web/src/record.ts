/** Recording a noise from the microphone, as a WAV file the server measures (see crates/hoardd/src/imitation.rs). */

/** Whether this browser can record here: a microphone needs a secure page (HTTPS or localhost). */
export function canRecord(): boolean {
  return typeof MediaRecorder !== 'undefined' && !!navigator.mediaDevices?.getUserMedia
}

/** The longest recording, in milliseconds. */
const MAX_MS = 6000

export interface Recording {
  /** Stops recording; `done` then settles. */
  stop(): void
  /** The recording as WAV, once stopped (or after six seconds). */
  done: Promise<Blob>
}

/** Starts recording from the microphone, asking for it first if the browser hasn't been told. */
export async function record(): Promise<Recording> {
  // As the microphone hears it: noise suppression would take away a "shhh", and gain control the shape of a "pew".
  const stream = await navigator.mediaDevices.getUserMedia({
    audio: { echoCancellation: false, noiseSuppression: false, autoGainControl: false },
  })
  const recorder = new MediaRecorder(stream)
  const chunks: Blob[] = []
  recorder.ondataavailable = (e) => {
    if (e.data.size) chunks.push(e.data)
  }
  const stop = () => {
    if (recorder.state !== 'inactive') recorder.stop()
  }
  const timer = setTimeout(stop, MAX_MS)
  const done = new Promise<Blob>((resolve, reject) => {
    recorder.onstop = async () => {
      clearTimeout(timer)
      for (const track of stream.getTracks()) track.stop()
      const ctx = new AudioContext()
      try {
        const encoded = await new Blob(chunks, { type: recorder.mimeType }).arrayBuffer()
        resolve(wav(await ctx.decodeAudioData(encoded)))
      } catch (e) {
        reject(e)
      } finally {
        void ctx.close()
      }
    }
  })
  recorder.start()
  return { stop, done }
}

/** Mixed to mono, as 16-bit PCM WAV. */
function wav(audio: AudioBuffer): Blob {
  const n = audio.length
  const channels = Array.from({ length: audio.numberOfChannels }, (_, c) => audio.getChannelData(c))
  const out = new DataView(new ArrayBuffer(44 + n * 2))
  const text = (at: number, s: string) => [...s].forEach((ch, i) => out.setUint8(at + i, ch.charCodeAt(0)))
  text(0, 'RIFF')
  out.setUint32(4, 36 + n * 2, true)
  text(8, 'WAVEfmt ')
  out.setUint32(16, 16, true)
  out.setUint16(20, 1, true)
  out.setUint16(22, 1, true)
  out.setUint32(24, audio.sampleRate, true)
  out.setUint32(28, audio.sampleRate * 2, true)
  out.setUint16(32, 2, true)
  out.setUint16(34, 16, true)
  text(36, 'data')
  out.setUint32(40, n * 2, true)
  for (let i = 0; i < n; i++) {
    let s = 0
    for (const ch of channels) s += ch[i]
    s = Math.max(-1, Math.min(1, s / channels.length))
    out.setInt16(44 + i * 2, Math.round(s * 32767), true)
  }
  return new Blob([out.buffer], { type: 'audio/wav' })
}
