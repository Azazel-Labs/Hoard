// Searches HOARD_URL (with HOARD_TOKEN) for HOARD_QUERY, adds every match to the Lumina project at HOARD_PROJECT the
// way the Hoard drawer does (short of importing, which needs the editor), and prints the outcome as one
// `RESULT {json}` line. Built with the plugin's core and run by web/e2e/engines.spec.ts.
#define _CRT_SECURE_NO_WARNINGS
#include "HoardCore.h"

#include <cstdio>
#include <cstdlib>
#include <string>

using namespace Hoard;

static std::string Env(const char* Name)
{
    const char* V = std::getenv(Name);
    return V ? V : "";
}

static int Done(const Json& R)
{
    std::printf("RESULT %s\n", R.Write().c_str());
    const Json* Ok = R.Find("ok");
    return Ok && Ok->Bool ? 0 : 1;
}

int main()
{
    Client With;
    With.Server = Env("HOARD_URL");
    With.Token = Env("HOARD_TOKEN");
    const Reply Found = With.Search(Env("HOARD_QUERY"), Env("HOARD_KIND"), 0, 40);
    Json R = Json::Object();
    if (!Found.bOk)
    {
        R.Set("ok", Json::Boolean(false));
        R.Set("error", Json::Str(Found.Error));
        return Done(R);
    }
    const std::vector<Json>& Items = Found.Data.List("items");
    int Thumbs = 0;
    for (const Json& A : Items)
    {
        // What the drawer shows: PNG or JPEG, which is what Lumina's texture loader reads.
        const Json* Has = A.Find("has_thumb");
        const std::string B = Has && Has->Bool ? With.Thumbnail(A) : std::string();
        const auto At = [&](size_t I) { return static_cast<unsigned char>(B[I]); };
        if (B.size() > 3 && ((At(0) == 0x89 && At(1) == 0x50) || (At(0) == 0xFF && At(1) == 0xD8)))
        {
            ++Thumbs;
        }
    }
    const Project Proj(Env("HOARD_PROJECT"), "Hoard");
    const AddResult Added = Proj.Add(With, Items);
    R.Set("ok", Json::Boolean(Added.bOk));
    R.Set("error", Added.Error.empty() ? Json() : Json::Str(Added.Error));
    R.Set("files", Json::Int(Added.Files));
    R.Set("fetched", Json::Int(static_cast<int64_t>(Added.Downloaded.size())));
    R.Set("reported", Json::Boolean(Added.bReported));
    R.Set("matched", Json::Int(static_cast<int64_t>(Items.size())));
    R.Set("thumbnails", Json::Int(Thumbs));
    return Done(R);
}
