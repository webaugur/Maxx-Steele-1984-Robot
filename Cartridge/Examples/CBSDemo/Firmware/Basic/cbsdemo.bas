REM CBS Toys factory demo cartridge (maxx_demo_ROM_532.dsm)
REM Compile:
REM   maxx compile cbsdemo.bas -o ../Binary/CBSDemo.bas.532 --copyright cbs \
REM     --tables-from ../Binary/CBSDemo.532
REM
REM SAY phrases use RAM slots from the factory phrase table ($A081).
REM Use --tables-from CBSDemo.532 so custom speech bytes are preserved.

10  REM --- Speech and delays ---
20  SAY 16          REM "Hello, I am Maxx Steele"
30  DELAY 2
40  SAY 0           REM "I am great, and you"
50  DELAY 10
60  SAY 22          REM "Good morning"
70  DELAY 1
80  SAY 23          REM "It is time to get up"
90  DELAY 1
100 PLAY 6          REM Reveille
110 DELAY 10
120 SAY 1           REM "I am ready when you are"
130 DELAY 1

140 REM --- Motion sequence ---
150 FORWARD 20
160 DELAY 4
170 ARMS UP 40
180 WRIST DOWN 35
190 DELAY 3
200 CLAW ROTATE 21
210 CLAW OPEN
220 DELAY 2
230 LAMP ON
240 DELAY 7
250 LAMP OFF
260 RIGHT 6
270 DELAY 5

280 REM --- More speech ---
290 SPEAK 63        REM "Ha ha ha ha ha" (ROM phrase)
300 DELAY 4
310 SAY 2           REM "I am a great match for humans"
320 DELAY 3
330 SAY 3           REM "Goodbye for now, have a good day"
340 DELAY 1

350 REM --- Finish ---
360 LEFT 5
370 DELAY 2
380 PLAY 0
390 DELAY 2
400 HOME
410 BACK 20
420 END