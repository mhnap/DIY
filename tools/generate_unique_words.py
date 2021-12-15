#!/usr/bin/env python3

import sys
import random  
import string  


def main(app_name, argv):
  if len(argv) < 2:
    print("{} filename size".format(app_name))
    return 1

  filename = argv[0]
  bytes_to_write = int(argv[1])

  word_lengths = list(range(1, 12))

  bytes_written = 0
  words_written = 0

  f = open(filename, 'w')
  while bytes_written < bytes_to_write:
    word = random_string(random.choice(word_lengths))
    bytes_written += f.write(word)
    bytes_written += f.write(' ')

    words_written += 1
  f.close()

  print("Words written: {}. Bytes written: {}".format(words_written, bytes_written))


def random_string(length):
  return ''.join((random.choice(string.ascii_lowercase) for x in range(length)))

if __name__ == "__main__":
  ret = main(sys.argv[0], sys.argv[1:])
  sys.exit(ret)
