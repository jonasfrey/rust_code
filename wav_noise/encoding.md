
1 - 4	“RIFF”	Marks the file as a riff file. Characters are each 1 byte long.

5 - 8	File size (integer)	Size of the overall file - 8 bytes, in bytes (32-bit integer)
. Typically, you’d fill this in after creation.

9 -12	“WAVE”	File Type Header. For our purposes, it always equals “WAVE”.

13-16	“fmt "	Format chunk marker. Includes trailing null

17-20	16	Length of format data as listed above

21-22	1	Type of format (1 is PCM) - 2 byte integer

23-24	2	Number of Channels - 2 byte integer

25-28	44100	Sample Rate - 32 byte integer. Common values are 44100 (CD), 48000 (DAT). 
Sample Rate = Number of Samples per second, or Hertz.

29-32	176400	(Sample Rate * BitsPerSample * Channels) / 8.

    33-34	4	(BitsPerSample * Channels) / 8.1 - 8 bit mono2 - 8 bit stereo/16 bit mono4 - 
    16 bit stereo

35-36	16	Bits per sample

37-40	“data”	“data” chunk header. Marks the beginning of the data section.

41-44	File size (data)	Size of the data section.

Sample values are given above for a 16-bit stereo source.








The canonical WAVE format starts with the RIFF header:

0         4   ChunkID          Contains the letters "RIFF" in ASCII form
                               (0x52494646 big-endian form).
4         4   ChunkSize        36 + SubChunk2Size, or more precisely:
                               4 + (8 + SubChunk1Size) + (8 + SubChunk2Size)
                               This is the size of the rest of the chunk 
                               following this number.  This is the size of the 
                               entire file in bytes minus 8 bytes for the
                               two fields not included in this count:
                               ChunkID and ChunkSize.
8         4   Format           Contains the letters "WAVE"
                               (0x57415645 big-endian form).

The "WAVE" format consists of two subchunks: "fmt " and "data":
The "fmt " subchunk describes the sound data's format:

12        4   Subchunk1ID      Contains the letters "fmt "
                               (0x666d7420 big-endian form).
16        4   Subchunk1Size    16 for PCM.  This is the size of the
                               rest of the Subchunk which follows this number.
20        2   AudioFormat      PCM = 1 (i.e. Linear quantization)
                               Values other than 1 indicate some 
                               form of compression.
22        2   NumChannels      Mono = 1, Stereo = 2, etc.
24        4   SampleRate       8000, 44100, etc.
28        4   ByteRate         == SampleRate * NumChannels * BitsPerSample/8
32        2   BlockAlign       == NumChannels * BitsPerSample/8
                               The number of bytes for one sample including
                               all channels. I wonder what happens when
                               this number isn't an integer?
34        2   BitsPerSample    8 bits = 8, 16 bits = 16, etc.
          2   ExtraParamSize   if PCM, then doesn't exist
          X   ExtraParams      space for extra parameters

The "data" subchunk contains the size of the data and the actual sound:

36        4   Subchunk2ID      Contains the letters "data"
                               (0x64617461 big-endian form).
40        4   Subchunk2Size    == NumSamples * NumChannels * BitsPerSample/8
                               This is the number of bytes in the data.
                               You can also think of this as the size
                               of the read of the subchunk following this 
                               number.
44        *   Data             The actual sound data.