@echo off
call build.cmd

set prg=.\target\debug\uniq.exe

echo a>test.fic
echo c>>test.fic
echo b>>test.fic
echo a>>test.fic
echo c>>test.fic
echo A>>test.fic
%prg%  test.fic
echo expected: a b c A
pause

%prg% -c  test.fic
echo expected: a 2     b 1     c 2     A 1
pause



%prg% -i test.fic
echo expected: a b c 
pause

%prg% -c -i test.fic
echo expected: a 3     b 1     c 2
pause
