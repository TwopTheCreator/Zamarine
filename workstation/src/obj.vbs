Option Explicit

Dim fso, backendPath, sessionPath, backendContent, sessionContent

Set fso = CreateObject("Scripting.FileSystemObject")

backendPath = "C:..\workstation\workstation.py"
sessionPath = "C:..\workstation\session.py"

If Not fso.FolderExists("C:\Workstation") Then
    fso.CreateFolder "C:\Workstation"
End If

backendContent = "import os" & vbCrLf & _
"" & vbCrLf & _
"print('Workstation backend initialized')"

sessionContent = "import json" & vbCrLf & _
"" & vbCrLf & _
"print('Session manager initialized')"

WriteFile backendPath, backendContent

WriteFile sessionPath, sessionContent

WScript.Echo "Workstation objects placed successfully."

Sub WriteFile(path, content)
    Dim file
    Set file = fso.CreateTextFile(path, True)
    file.Write content
    file.Close
End Sub
