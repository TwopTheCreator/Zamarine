#NoEnv
SendMode Input
SetWorkingDir %A_ScriptDir%

F1::
    Run, notepad.exe
return

F2::
    Run, calc.exe
return

F3::
    DllCall("LockWorkStation")
return

MButton::
    Run, taskmgr.exe
return

RButton::
    Run, explorer.exe
return

Esc::ExitApp
