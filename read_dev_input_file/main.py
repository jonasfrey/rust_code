import struct 
f = open( "/dev/input/event18", "rb" ); 
# Open the file in the read-binary mode
while 1:
  data = f.read(3)  # Reads the 3 bytes 
  print(struct.unpack('3b',data))  #Unpacks the bytes to integers