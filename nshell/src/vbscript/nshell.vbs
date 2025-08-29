' nshell.vbs - Windows Integration Component for nshell
' Provides Windows-specific functionality for the nshell environment

Option Explicit

Class NShell
    Private Sub Class_Initialize()
    End Sub
    
    Public Function GetSystemInfo()
        Dim objWMIService, colItems, objItem, info
        Set objWMIService = GetObject("winmgmts:\\\.\root\cimv2")
        Set colItems = objWMIService.ExecQuery("Select * from Win32_OperatingSystem",,48)
        
        For Each objItem in colItems
            info = "OS Name: " & objItem.Caption & vbCrLf
            info = info & "Version: " & objItem.Version & vbCrLf
            info = info & "Build: " & objItem.BuildNumber
        Next
        
        GetSystemInfo = info
    End Function
    
    Public Function RunCommand(cmd)
        Dim objShell, objExec, output
        Set objShell = CreateObject("WScript.Shell")
        
        On Error Resume Next
        Set objExec = objShell.Exec("cmd /c " & cmd)
        
        If Err.Number <> 0 Then
            RunCommand = "Error: " & Err.Description
            Exit Function
        End If
        
        output = objExec.StdOut.ReadAll()
        If output = "" Then output = objExec.StdErr.ReadAll()
        
        RunCommand = output
    End Function
    
    Public Function GetProcessList()
        Dim objWMIService, colItems, objItem, procList
        Set objWMIService = GetObject("winmgmts:\\\.\root\cimv2")
        Set colItems = objWMIService.ExecQuery("Select * from Win32_Process")
        
        procList = "Running Processes:" & vbCrLf & String(50, "-") & vbCrLf
        procList = procList & "PID" & vbTab & "Process Name" & vbCrLf & String(50, "-") & vbCrLf
        
        For Each objItem in colItems
            procList = procList & objItem.ProcessId & vbTab & objItem.Name & vbCrLf
        Next
        
        GetProcessList = procList
    End Function
    
    Public Function GetDiskInfo()
        Dim objWMIService, colItems, objItem, diskInfo
        Set objWMIService = GetObject("winmgmts:\\\.\root\cimv2")
        Set colItems = objWMIService.ExecQuery("Select * from Win32_LogicalDisk")
        
        diskInfo = "Disk Information:" & vbCrLf & "-----------------" & vbCrLf
        
        For Each objItem in colItems
            diskInfo = diskInfo & objItem.DeviceID & " - " & _
                      FormatNumber(objItem.FreeSpace / 1073741824, 1) & "GB free of " & _
                      FormatNumber(objItem.Size / 1073741824, 1) & "GB" & vbCrLf
        Next
        
        GetDiskInfo = diskInfo
    End Function
End Class
