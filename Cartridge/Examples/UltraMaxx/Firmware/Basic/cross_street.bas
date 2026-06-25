REM Cross the street — look both ways, roll across, play a tune
REM
REM Compile (music table from stock UltraMaxx cart):
REM   maxx compile cross_street.bas -o ../Binary/cross_street.532 --copyright ultramaxx \
REM     --tables-from ../Binary/UltraMaxx.532
REM
REM Simulate:
REM   maxx simulate cross_street.bas --tables-from ../Binary/UltraMaxx.532
REM   maxx simulate cross_street.bas --gui --tables-from ../Binary/UltraMaxx.532

10  REM --- Stop at the curb ---
20  DELAY 1

30  REM --- Look left ---
40  LEFT 30
50  DELAY 2

60  REM --- Look right (sweep past forward) ---
70  RIGHT 60
80  DELAY 2

90  REM --- Face forward again ---
100 LEFT 30
110 DELAY 1

120 REM --- Cross the street ---
130 LAMP ON
140 FORWARD 20
150 DELAY 2
160 LAMP OFF

170 REM --- Celebrate on the other side ---
180 PLAY 6          REM Reveille (tune #6 on demo cart)
190 DELAY 10
200 HOME
210 END