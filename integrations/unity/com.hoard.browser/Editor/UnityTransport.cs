// The package's requests over UnityWebRequest, which every Unity has whatever its API compatibility level, and which
// finishes on the main thread, where the window and the AssetDatabase can be touched.
using System.Threading.Tasks;
using UnityEngine.Networking;

namespace Hoard
{
    sealed class UnityTransport : ITransport
    {
        public Task<HttpAnswer> Send(HttpCall call)
        {
            var done = new TaskCompletionSource<HttpAnswer>();
            var req = new UnityWebRequest(call.Url, call.Method);
            if (call.Body != null) req.uploadHandler = new UploadHandlerRaw(call.Body);
            req.downloadHandler = call.DownloadTo != null
                ? (DownloadHandler)new DownloadHandlerFile(call.DownloadTo) { removeFileOnAbort = true }
                : new DownloadHandlerBuffer();
            foreach (var h in call.Headers) req.SetRequestHeader(h.Key, h.Value);
            req.timeout = call.TimeoutSeconds;
            req.SendWebRequest().completed += _ =>
            {
                var answer = new HttpAnswer { Code = (int)req.responseCode };
                if (req.result == UnityWebRequest.Result.ConnectionError || req.result == UnityWebRequest.Result.DataProcessingError)
                    answer.Error = req.error;
                else if (call.DownloadTo == null)
                    answer.Body = req.downloadHandler.data ?? System.Array.Empty<byte>();
                req.Dispose();
                done.SetResult(answer);
            };
            return done.Task;
        }
    }
}
