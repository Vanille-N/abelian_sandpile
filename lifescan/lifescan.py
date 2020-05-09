#!/usr/bin/python3

import pyautogui as gui
import time as t

gui.FAILSAFE = True
gui.PAUSE = 0.02


def countdown(n):
    if (n == 0):
        return
    for k in range(n, 0, -1):
        print("{}...".format(k))
        t.sleep(1)
    print("CLICK !")

def calibrate_zone():
    print("Upper left corner (please center within the cell)")
    countdown(5)
    pos_ul = gui.position()
    print("Lower right corner (please center within the cell)")
    countdown(5)
    pos_dr = gui.position()
    return (pos_ul, pos_dr)

def calibrate_color(type):
    scr = gui.screenshot()
    print("Please select {} cell".format(type))
    countdown(5)
    color = scr.getpixel(gui.position())
    return color

def color_distance(a, b):
    return ((a[0]-b[0])**2 + (a[1]-b[1])**2 + (a[2]-b[2])**2)

def identify_color(color, ref):
    min_dist = 100000
    min_idx = -1
    for i, c in enumerate(ref):
        d = color_distance(c, color)
        if (d < min_dist):
            min_dist = d
            min_idx = i
    return min_idx

def main():
    ul, dr = calibrate_zone()
    cells = [calibrate_color("dead"), calibrate_color("live")]
    scr = gui.screenshot()
    hgt = int(input("Height ? "))
    wth = int(input("Width ? "))
    for ifrac in range(hgt):
        for jfrac in range(wth):
            i = ul.y + (dr.y - ul.y) * ifrac / (hgt-1)
            j = ul.x + (dr.x - ul.x) * jfrac / (wth-1)
            gui.moveTo(j, i)
            print(".x"[identify_color(scr.getpixel((j, i)), cells)], end="")
        print()

main()
