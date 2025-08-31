using System;
using System.Runtime.InteropServices;

public static class Fabric
{
    [DllImport("fabric", CallingConvention = CallingConvention.Cdecl)]
    private static extern bool fabric_init();

    [DllImport("fabric", CallingConvention = CallingConvention.Cdecl)]
    private static extern bool fabric_index_data(string key, byte[] data, int length);

    [DllImport("fabric", CallingConvention = CallingConvention.Cdecl)]
    private static extern bool fabric_search(string query, out IntPtr result);

    [DllImport("fabric", CallingConvention = CallingConvention.Cdecl)]
    private static extern void fabric_free_string(IntPtr str);

    public static bool Init()
    {
        return fabric_init();
    }

    public static bool IndexData(string key, byte[] data)
    {
        return fabric_index_data(key, data, data.Length);
    }

    public static string Search(string query)
    {
        if (fabric_search(query, out IntPtr result))
        {
            string str = Marshal.PtrToStringAnsi(result);
            fabric_free_string(result);
            return str;
        }
        return null;
    }
}
