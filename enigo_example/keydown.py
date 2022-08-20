import os
import keyboard  # using module keyboard
import subprocess
import time
n_i = 0
while True:  # making a loop
    n_i += 1
    time.sleep(0.01)
    print(n_i)
    try:  # used try so that if user pressed other than the given key error will not be shown
        if keyboard.is_pressed('shift'):  # if key 'q' is pressed 
            subprocess.run(["./target/debug/enigo_example"])
            # print('You Pressed shift Key!')
            # break  # finishing the loop
    except:
        # print("asdf")
        continue  # if user pressed a key other than the given key the loop will break

