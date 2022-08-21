import os
import keyboard  # using module keyboard
import pyautogui
import subprocess
import time
import json
n_i = 0
# subprocess.run(["cargo", "build"])

# subprocess.run(["cargo", "build"])
class O_rust_binary_parameter():
  def __init__(
    self, 
    s_function_name,
    n_x, 
    n_y,
  ):
    self.s_function_name = s_function_name
    self.n_x = n_x
    self.n_y = n_y

o_rust_binary_parameter = O_rust_binary_parameter("f_click", 0, 0)

class O_mouse_position():
  def __init__(
    self, 
    n_x, 
    n_y,
    n_index
  ):
    self.n_index = n_index
    self.n_x = n_x
    self.n_y = n_y

a_o_mouse_position = []
n_i = 0
while(n_i < 10):
    a_o_mouse_position.append(
        O_mouse_position(
            0, 0, n_i
        )
    )
    n_i +=1

if os.geteuid() != 0:
    print("run as root!")
    exit()

while True:  # making a loop
    n_i += 1
    if n_i % 20 == 0:
        print(str(__file__) + " is still running!, hit 'ctrl+escape' key to end")
    time.sleep(0.01)
    try:  # used try so that if user pressed other than the given key error will not be shown
        if keyboard.is_pressed('shift'):  # if key 'q' is pressed 
            o_rust_binary_parameter.s_function_name = "f_click"
            s_json = json.dumps(o_rust_binary_parameter.__dict__)
            subprocess.run(["./target/debug/enigo_example", s_json])

        if keyboard.is_pressed("ctrl"):  # if key 'q' is pressed 
            # ctrl + alt + [1||2||3||4||5...] == store current mouse position
            # ctrl + [1||2||3||4||5...] == move mouse to stored position
            o_rust_binary_parameter.s_function_name = "f_move"
            if keyboard.is_pressed(
                "alt"
            ):
                a_mouse_pos = pyautogui.position()
                for o_mouse_position in a_o_mouse_position: 
                    if(keyboard.is_pressed(str(o_mouse_position.n_index))):
                        # print(str(o_mouse_position.n_index))
                        o_mouse_position.n_x = a_mouse_pos[0]
                        o_mouse_position.n_y = a_mouse_pos[1]
                        print(f"w mousepos: {o_mouse_position.n_x}|{o_mouse_position.n_y} to ctrl+{str(o_mouse_position.n_index)}")

            if keyboard.is_pressed(
                "alt"
            ) == False: 
                for o_mouse_position in a_o_mouse_position: 
                    if(keyboard.is_pressed(str(o_mouse_position.n_index))):
                        o_rust_binary_parameter.n_x = o_mouse_position.n_x
                        o_rust_binary_parameter.n_y = o_mouse_position.n_y
                        print(f"r mousepos: {o_mouse_position.n_x}|{o_mouse_position.n_y} with ctrl+alt+{str(o_mouse_position.n_index)}")

                        subprocess.run(["./target/debug/enigo_example", json.dumps(o_rust_binary_parameter.__dict__)])
            # print('You Pressed shift Key!')
            # break  # finishing the loop
        if keyboard.is_pressed('ctrl+escape'):  # if key 'q' is pressed 
            print("'ctrl+escape' key was hit, end")
            break
    except:
        # print("asdf")
        continue  # if user pressed a key other than the given key the loop will break