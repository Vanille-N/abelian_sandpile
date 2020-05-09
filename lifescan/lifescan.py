#!/usr/bin/python3

import pyautogui as gui
import time as t
import random as rnd

gui.FAILSAFE = True
gui.PAUSE = 0.0

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

def randomize(x, y, radius):
    #print("a", x, y)
    x += rnd.random()*radius
    y += rnd.random()*radius
    gui.moveTo(x, y)
    #print("b", x, y)
    return (x, y)

def majority(sample, chars):
    cnt = [[0, c] for c in chars]
    for s in sample:
        cnt[s][0] += 1
    cnt.sort(reverse=True)
    #print(cnt)
    if cnt[0][0] > cnt[1][0] * 2:
        return cnt[0][1]
    else:
        return input("Unsure: {}, please resolve manually: ".format(cnt))

def main():
    ul, dr = calibrate_zone()
    cells = [calibrate_color("dead"), calibrate_color("live")]
    print(cells)
    scr = gui.screenshot()
    hgt = int(input("Height ? "))
    wth = int(input("Width ? "))
    radius = min(hgt / abs(dr.y - ul.y) / 3, wth / abs(dr.x - ul.x) / 3)
    print("radius: {}".format(radius))
    for ifrac in range(hgt):
        for jfrac in range(wth):
            i = ul.y + (dr.y - ul.y) * ifrac / (hgt-1)
            j = ul.x + (dr.x - ul.x) * jfrac / (wth-1)
            gui.moveTo(j, i)
            sample = [identify_color(scr.getpixel(randomize(j, i, radius)), cells) for _ in range(10)]
            print(majority(sample, ".x"), end="")
        print()

main()
