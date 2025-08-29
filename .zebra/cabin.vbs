Option Explicit

Dim objFSO, objShell, jsonPath, jsonData

Set objFSO = CreateObject("Scripting.FileSystemObject")
Set objShell = CreateObject("WScript.Shell")

jsonPath = "C:\Zamarine\.zebra\zebra.json"

If objFSO.FileExists(jsonPath) Then
    Dim objFile
    Set objFile = objFSO.OpenTextFile(jsonPath, 1)
    jsonData = objFile.ReadAll
    objFile.Close

    WScript.Echo "Zebra JSON loaded successfully."
Else
    WScript.Echo "Error: Zebra JSON file not found at " & jsonPath
    WScript.Quit
End If

Dim objOS
Set objOS = CreateObject("WScript.Network")

WScript.Echo "Computer Name: " & objOS.ComputerName
WScript.Echo "User Name: " & objOS.UserName

Dim tempFile
tempFile = objShell.ExpandEnvironmentStrings("%TEMP%") & "\zebra_os_temp.json"

Dim outFile
Set outFile = objFSO.CreateTextFile(tempFile, True)
outFile.Write jsonData
outFile.Close

WScript.Echo "Zebra JSON connected to OS object via Cabin."

' === Cleanup ===
Set objFSO = Nothing
Set objShell = Nothing
Set objOS = Nothing
