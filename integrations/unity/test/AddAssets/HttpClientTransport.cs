// The package's requests over HttpClient, standing in for UnityWebRequest outside Unity.
using System;
using System.IO;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using Hoard;

sealed class HttpClientTransport : ITransport
{
    static readonly HttpClient Http = new HttpClient { Timeout = Timeout.InfiniteTimeSpan };

    public async Task<HttpAnswer> Send(HttpCall call)
    {
        using var cts = call.TimeoutSeconds > 0 ? new CancellationTokenSource(TimeSpan.FromSeconds(call.TimeoutSeconds)) : new CancellationTokenSource();
        using var req = new HttpRequestMessage(new HttpMethod(call.Method), call.Url);
        if (call.Body != null) req.Content = new ByteArrayContent(call.Body);
        foreach (var h in call.Headers)
        {
            if (h.Key == "Content-Type") req.Content?.Headers.TryAddWithoutValidation(h.Key, h.Value);
            else req.Headers.TryAddWithoutValidation(h.Key, h.Value);
        }
        try
        {
            using var res = await Http.SendAsync(req, HttpCompletionOption.ResponseHeadersRead, cts.Token);
            var code = (int)res.StatusCode;
            if (call.DownloadTo != null && res.IsSuccessStatusCode)
            {
                using (var from = await res.Content.ReadAsStreamAsync(cts.Token))
                using (var to = File.Create(call.DownloadTo))
                    await from.CopyToAsync(to, cts.Token);
                return new HttpAnswer { Code = code };
            }
            return new HttpAnswer { Code = code, Body = await res.Content.ReadAsByteArrayAsync(cts.Token) };
        }
        catch (OperationCanceledException)
        {
            return new HttpAnswer { Error = "it took too long to answer" };
        }
        catch (HttpRequestException e)
        {
            return new HttpAnswer { Error = (e.InnerException ?? e).Message };
        }
        catch (IOException e)
        {
            return new HttpAnswer { Error = e.Message };
        }
    }
}
