#!/usr/bin/python3

print("MDLChef MEME FORMATS MANAGEMENT GUI")

import tkinter as tk
from PIL import ImageTk, Image, ImageDraw
from tkinter import simpledialog, messagebox
import sys
import json
from os import path

window = tk.Tk()

filepath = sys.argv[1]
print("Opening file:", filepath)
with Image.open(filepath) as image:
    newheight = 600
    ratio = newheight / image.height
    newwidth = image.width * ratio
    image = image.resize((int(newwidth), int(newheight)))

    imagegui = ImageTk.PhotoImage(image)
    imagelabel = tk.Label(image=imagegui)
    imagelabel.pack()

    text_box = tk.Message(text = "Click to make meme inserts. Ctrl+S to save and exit.", width = 500)
    text_box.pack()

    firstclick = True
    coord1 = None
    coord2 = None
    memelabels = {}
    def imageleftclick(event):
        global firstclick
        global coord1
        global coord2
        global image
        global imagegui
        if firstclick:
            coord1 = (event.x, event.y)
        else:
            coord2 = (event.x, event.y)
            if coord1[0] > coord2[0]:
                tmp = coord1
                coord1 = coord2
                coord2 = tmp
            print(coord1, coord2)
            draw = ImageDraw.Draw(image)
            draw.rectangle([coord1, coord2], f"hsl({len(memelabels) * 45}, 100%, 50%)")
            imagegui = ImageTk.PhotoImage(image)
            imagelabel.configure(image=imagegui)
            userinp = simpledialog.askstring("Insert Label", "What should this insert be called?")
            if userinp is None or len(userinp) == 0 or ' ' in userinp:
                print(f"Error! Bad label identifier '{userinp}'.")
            else:
                #text_box.configure(text = text_box.cget("text") + f"\n{userinp}: {coord1}; {coord2}")
                memelabels[userinp] = {"coords": (tuple([int(a//ratio) for a in coord1]),
                                                  tuple([int(b//ratio) for b in coord2]))}
                print(memelabels)
                text_box.configure(text = json.dumps(memelabels))
        firstclick = not firstclick

    imagelabel.bind("<Button-1>", imageleftclick)

    def save_and_exit(event):
        mememeta = {"inserts": memelabels}
        mememetajson = json.dumps(mememeta, indent = 2)
        newfilepath = path.splitext(filepath)[0] + ".meme"
        print(mememetajson)
        print(newfilepath)
        exists = ""
        if path.exists(newfilepath):
            exists = "\n\nWARNING: FILE ALREADY EXISTS! WILL OVERWRITE"
        answer = messagebox.askokcancel("Save Meme Meta", f"Do you want to save as\n{newfilepath}\nPlease confirm.{exists}")
        if answer:
            print("Saving!")
            outfile = open(newfilepath, "w")
            outfile.write(mememetajson)
            outfile.close()
        else:
            print("Cancel. Did not save.")
        window.destroy()

    window.bind_all("<Control-Key-s>", save_and_exit)

    window.mainloop()

print("Quitting.")
