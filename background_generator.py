import random

width = 1000
height = 50
characters = "✦★•°☆"

with open("unicode_art.txt", "w", encoding="utf-8") as f:
    for _ in range(height):
        line = ""
        for _ in range(width):
            line += " " * random.randint(0, 100) + random.choice(characters) + " " * random.randint(0, 100)
        finished_line = line[:width]
        f.write(finished_line + "\n")

print("Unicode art generated and saved to unicode_art.txt")
