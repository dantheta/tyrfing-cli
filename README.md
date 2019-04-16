
tyrfing-cli
===========

Command-line interactive debug tool for Drevo Tyrfing v2 keyboard.

The notes here are a work in progress and very rough.

Usage
-----

(as root, or with write permission for /dev/hidrawX
tyrfing-cli /dev/hidrawX

Finding the hidraw device
-------------------------

```
$ ls /sys/bus/hid/devices/*\:0416\:A0F8.0059/hidraw/
hidraw5
## so use:
$ ./tyrfing-cli /dev/hidraw5
```

Using the interactive tool

```
  6  190   21    0    1    1    3    1  
  5    1    0    0    0    0    0    0  
255    0    0    1    0    0    0    0  
  0    0    0    0    0    0    0    0
Enter 'h' for help
>
```

The interactive tool accepts 4 commands:

* p: print
* w: write
* r: reset
* q: quit
* numbers 0-31: set new value.

The configuration message for the keyboard contains 32 bytes.  These
byte values are manipulated by the tool and written to the keyboard when
the 'w' command is given.

Example
-------

To change the mode to breathe in blue, enter:

```
> 6   # change byte 6
? 2   # new value
> 19  # change byte 19
? 0   # set to 0 (single color)
> 18  # change byte 18 
? 255 # set to 255 (blue bg)
> 16  # change byte 16
? 0   # set red to 0
> w
```

Interesting byte values
-----------------------

byte 6: pattern
*    0 - ripple fill
*    2 - breathe
*    3 - stripe
*    4 - rain
*    5 - horse race
*    6 - disco floor
*    7 - key trail
*    8 - key ripple
*    9 - key strike
*    10 - key ripple 2
*    11 - riple and rain
*    12 - hstrike
*    13 - key fade
*    14 - off/custom 1?
*    15 - custom 2?
*    17 - radar
*    18 - rain again ?

byte 8:
*	low 4 bits: brightness
*	high bit: oscilate brightness?

byte 9:
*	low 2 bits: direction
	
byte 7:
*	0 - 9 speed

byte 12: bg red
byte 13: bg green
byte 14: bg blue
byte 15: bit 1: bg multi

byte 16: red
byte 17: green
byte 18: blue
byte 19: color
    bit1 : multi



