#!/bin/python

vertices = []
with open("file.txt","r") as file:
    for line in file.readlines():
        if line in vertices:
            print("aahhh")
        else:
            vertices.append(line)