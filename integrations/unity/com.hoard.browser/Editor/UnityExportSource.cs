// What sending to Hoard asks of the editor: what an asset uses (the AssetDatabase's dependencies), whether it was
// imported from a file other tools open (by its importer), and copies of meshes and textures made in Unity.
using System.Collections.Generic;
using System.Linq;
using UnityEditor;
using UnityEngine;

namespace Hoard
{
    sealed class UnityExportSource : IExportSource
    {
        public IEnumerable<string> InFolder(string folder) =>
            AssetDatabase.FindAssets("", new[] { folder })
                .Select(AssetDatabase.GUIDToAssetPath)
                .Where(p => !AssetDatabase.IsValidFolder(p))
                .Distinct();

        public IEnumerable<string> Dependencies(IList<string> assets) =>
            assets.Count == 0 ? Enumerable.Empty<string>() : assets.Concat(AssetDatabase.GetDependencies(assets.ToArray(), true)).Distinct();

        public bool IsImported(string asset)
        {
            var importer = AssetImporter.GetAtPath(asset);
            return importer is ModelImporter || importer is TextureImporter || importer is AudioImporter || importer is TrueTypeFontImporter ||
                importer is VideoClipImporter;
        }

        public ExportCopy Copy(string asset)
        {
            switch (AssetDatabase.LoadMainAssetAtPath(asset))
            {
                case Mesh mesh:
                    return MeshCopy(mesh);
                case Texture2D texture:
                    return new ExportCopy { Extension = ".png", Bytes = Png(texture, texture.width, texture.height) };
                case RenderTexture target:
                    return new ExportCopy { Extension = ".png", Bytes = Png(target, target.width, target.height) };
                default:
                    return null;
            }
        }

        static ExportCopy MeshCopy(Mesh mesh)
        {
            var vertices = mesh.vertices;
            if (vertices.Length == 0) return null;
            var positions = new float[vertices.Length * 3];
            for (var i = 0; i < vertices.Length; i++)
            {
                positions[i * 3] = vertices[i].x;
                positions[i * 3 + 1] = vertices[i].y;
                positions[i * 3 + 2] = vertices[i].z;
            }
            var n = mesh.normals;
            var normals = new float[n.Length * 3];
            for (var i = 0; i < n.Length; i++)
            {
                normals[i * 3] = n[i].x;
                normals[i * 3 + 1] = n[i].y;
                normals[i * 3 + 2] = n[i].z;
            }
            var uv = mesh.uv;
            var uvs = new float[uv.Length * 2];
            for (var i = 0; i < uv.Length; i++)
            {
                uvs[i * 2] = uv[i].x;
                uvs[i * 2 + 1] = uv[i].y;
            }
            var submeshes = new List<int[]>();
            for (var s = 0; s < mesh.subMeshCount; s++)
                if (mesh.GetTopology(s) == MeshTopology.Triangles) submeshes.Add(mesh.GetTriangles(s));
            return new ExportCopy { Extension = ".obj", Bytes = HoardExport.Obj(mesh.name, positions, normals, uvs, submeshes) };
        }

        /// <summary>A texture as PNG, by drawing it into a render target and reading that back, so one that's compressed
        /// or not readable from scripts comes out too.</summary>
        static byte[] Png(Texture texture, int width, int height)
        {
            if (width <= 0 || height <= 0) return null;
            var target = RenderTexture.GetTemporary(width, height, 0, RenderTextureFormat.ARGB32, RenderTextureReadWrite.sRGB);
            var before = RenderTexture.active;
            var copy = new Texture2D(width, height, TextureFormat.RGBA32, false);
            try
            {
                Graphics.Blit(texture, target);
                RenderTexture.active = target;
                copy.ReadPixels(new Rect(0, 0, width, height), 0, 0);
                copy.Apply();
                return copy.EncodeToPNG();
            }
            finally
            {
                RenderTexture.active = before;
                RenderTexture.ReleaseTemporary(target);
                Object.DestroyImmediate(copy);
            }
        }
    }
}
