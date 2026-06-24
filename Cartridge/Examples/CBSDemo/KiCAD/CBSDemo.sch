EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A3 16535 11693
encoding utf-8
Sheet 1 1
Title "Maxx Steele CBSDemo Cartridge"
Date "2026-06-24"
Rev "0.1"
Comp "Maxx-Steele-1984-Robot"
Comment1 "Reverse-engineered from maxxcard.jpg"
Comment2 "Provisional netlist - see KiCAD/trace-worksheet.md"
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Connector:Conn_01x44_Pin J1
U 1 1 A8768F10
P 1200 3600
F 0 "J1" H 1200 3450 50  0000 C CNN
F 1 "MaxxCard_Edge_44x2.54mm" H 1200 3750 50  0000 C CNN
F 2 "Connector:Conn_01x44_Pin" H 1200 3600 50  0001 C CNN
F 3 "~" H 1200 3600 50  0001 C CNN
	1    1200 3600
	0    0    0    -1  
$EndComp
$Comp
L Memory_EPROM:27C512 U1
U 1 1 DF522A8A
P 3600 2200
F 0 "U1" H 3600 2050 50  0000 C CNN
F 1 "27C512" H 3600 2350 50  0000 C CNN
F 2 "Package_DIP:DIP-28_W15.24mm" H 3600 2200 50  0001 C CNN
F 3 "${KIPRJMOD}/../../../../DataSheets/Mitsubishi-KM2365.pdf" H 3600 2200 50  0001 C CNN
	1    3600 2200
	0    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x24_Pin U3
U 1 1 BAE33EA3
P 5600 2200
F 0 "U3" H 5600 2050 50  0000 C CNN
F 1 "5085-TBD" H 5600 2350 50  0000 C CNN
F 2 "Package_DIP:DIP-24_W15.24mm" H 5600 2200 50  0001 C CNN
F 3 "~" H 5600 2200 50  0001 C CNN
	1    5600 2200
	0    0    0    -1  
$EndComp
$Comp
L 74xx:74HC14 U2
U 1 1 9F77AADA
P 7600 2200
F 0 "U2" H 7600 2050 50  0000 C CNN
F 1 "74HC14?" H 7600 2350 50  0000 C CNN
F 2 "Package_DIP:DIP-14_W7.62mm" H 7600 2200 50  0001 C CNN
F 3 "~" H 7600 2200 50  0001 C CNN
	1    7600 2200
	0    0    0    -1  
$EndComp
$Comp
L Device:C_Small C1
U 1 1 B4A705CC
P 3300 1700
F 0 "C1" H 3392 1746 50  0000 L CNN
F 1 "0.1uF" H 3392 1655 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D4.3mm_W1.9mm_P5.00mm" H 3300 1700 50  0001 C CNN
F 3 "~" H 3300 1700 50  0001 C CNN
	1    3300 1700
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C2
U 1 1 722B69A2
P 5300 1700
F 0 "C2" H 5392 1746 50  0000 L CNN
F 1 "0.1uF" H 5392 1655 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D4.3mm_W1.9mm_P5.00mm" H 5300 1700 50  0001 C CNN
F 3 "~" H 5300 1700 50  0001 C CNN
	1    5300 1700
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C3
U 1 1 221E8085
P 7300 1700
F 0 "C3" H 7392 1746 50  0000 L CNN
F 1 "0.1uF" H 7392 1655 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D4.3mm_W1.9mm_P5.00mm" H 7300 1700 50  0001 C CNN
F 3 "~" H 7300 1700 50  0001 C CNN
	1    7300 1700
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR01
U 1 1 41F62917
P 1200 5200
F 0 "#PWR01" H 1200 4950 50  0001 C CNN
F 1 "GND" H 1205 5027 50  0000 C CNN
F 2 "" H 1200 5200 50  0001 C CNN
F 3 "" H 1200 5200 50  0001 C CNN
	1    1200 5200
	1    0    0    -1  
$EndComp
$Comp
L power:+5V #PWR02
U 1 1 D1D11066
P 1600 5200
F 0 "#PWR02" H 1600 4950 50  0001 C CNN
F 1 "+5V" H 1605 5027 50  0000 C CNN
F 2 "" H 1600 5200 50  0001 C CNN
F 3 "" H 1600 5200 50  0001 C CNN
	1    1600 5200
	1    0    0    -1  
$EndComp
Wire Wire Line
	1600 5200 1600 1400
Wire Wire Line
	1600 1400 8200 1400
Wire Wire Line
	1200 5200 1200 3200
Wire Wire Line
	1200 3200 8200 3200
Wire Wire Line
	3300 1400 3300 1700
Wire Wire Line
	3300 1900 3300 3200
Wire Wire Line
	5300 1400 5300 1700
Wire Wire Line
	5300 1900 5300 3200
Wire Wire Line
	7300 1400 7300 1700
Wire Wire Line
	7300 1900 7300 3200
Wire Wire Line
	2000 4000 3200 4000
Wire Wire Line
	3200 4000 5400 4000
Wire Wire Line
	2000 3900 3200 3900
Wire Wire Line
	3200 3900 5400 3900
Wire Wire Line
	2000 3800 3200 3800
Wire Wire Line
	3200 3800 5400 3800
Wire Wire Line
	2000 3700 3200 3700
Wire Wire Line
	3200 3700 5400 3700
Wire Wire Line
	2000 3600 3200 3600
Wire Wire Line
	3200 3600 5400 3600
Wire Wire Line
	2000 3500 3200 3500
Wire Wire Line
	3200 3500 5400 3500
Wire Wire Line
	2000 3400 3200 3400
Wire Wire Line
	3200 3400 5400 3400
Wire Wire Line
	2000 3300 3200 3300
Wire Wire Line
	3200 3300 5400 3300
Wire Wire Line
	2000 3200 3200 3200
Wire Wire Line
	3200 3200 5400 3200
Wire Wire Line
	2000 3100 3200 3100
Wire Wire Line
	3200 3100 5400 3100
Wire Wire Line
	2000 3000 3200 3000
Wire Wire Line
	3200 3000 5400 3000
Wire Wire Line
	2000 2900 3200 2900
Wire Wire Line
	3200 2900 5400 2900
Wire Wire Line
	2000 2800 3200 2800
Wire Wire Line
	3200 2800 5400 2800
Wire Wire Line
	2000 2700 3200 2700
Wire Wire Line
	3200 2700 5400 2700
Wire Wire Line
	2000 2600 3200 2600
Wire Wire Line
	3200 2600 5400 2600
Wire Wire Line
	2000 2500 3200 2500
Wire Wire Line
	3200 2500 5400 2500
Wire Wire Line
	2000 5600 3200 5600
Wire Wire Line
	2000 5700 3200 5700
Wire Wire Line
	2000 5800 3200 5800
Wire Wire Line
	2000 5900 3200 5900
Wire Wire Line
	2000 6000 3200 6000
Wire Wire Line
	2000 6100 3200 6100
Wire Wire Line
	2000 6200 3200 6200
Wire Wire Line
	2000 6300 3200 6300
Wire Wire Line
	2000 2400 7400 2400
Wire Wire Line
	7400 2400 3200 2400
Wire Wire Line
	2000 2500 7400 2500
Wire Wire Line
	7400 2500 3200 2500
Wire Wire Line
	2000 5450 7400 5450
Wire Wire Line
	7400 5450 3200 5450
Wire Wire Line
	2000 5550 7400 5550
Wire Wire Line
	7400 5550 3200 5550
Text Label 2050 4000 0    50   ~ 0
A0
Text Label 2050 3900 0    50   ~ 0
A1
Text Label 2050 3800 0    50   ~ 0
A2
Text Label 2050 3700 0    50   ~ 0
A3
Text Label 2050 3600 0    50   ~ 0
A4
Text Label 2050 3500 0    50   ~ 0
A5
Text Label 2050 3400 0    50   ~ 0
A6
Text Label 2050 3300 0    50   ~ 0
A7
Text Label 2050 3200 0    50   ~ 0
A8
Text Label 2050 3100 0    50   ~ 0
A9
Text Label 2050 3000 0    50   ~ 0
A10
Text Label 2050 2900 0    50   ~ 0
A11
Text Label 2050 2800 0    50   ~ 0
A12
Text Label 2050 2700 0    50   ~ 0
A13
Text Label 2050 2600 0    50   ~ 0
A14
Text Label 2050 2500 0    50   ~ 0
A15
Text Label 2050 5600 0    50   ~ 0
D0
Text Label 2050 5700 0    50   ~ 0
D1
Text Label 2050 5800 0    50   ~ 0
D2
Text Label 2050 5900 0    50   ~ 0
D3
Text Label 2050 6000 0    50   ~ 0
D4
Text Label 2050 6100 0    50   ~ 0
D5
Text Label 2050 6200 0    50   ~ 0
D6
Text Label 2050 6300 0    50   ~ 0
D7
Text Label 2050 2400 0    50   ~ 0
/CE
Text Label 2050 2500 0    50   ~ 0
/OE
Text Label 2050 5450 0    50   ~ 0
PHI2
Text Label 2050 5550 0    50   ~ 0
R/W
$EndSCHEMATC
