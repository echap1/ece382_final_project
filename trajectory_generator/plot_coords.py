import csv

import matplotlib.pyplot as plt

x_values = []
y_values = []

u = 180

boundaries = [
    (-1, -1, 15, -1),
    (-1, -1, -1, 3),
    (-1, 1, 13, 1),
    (13, 1, 13, 3),
    (13, 3, 11, 3),
    (11, 3, 11, 5),
    (11, 5, 13, 5),
    (13, 5, 13, 9),
    (13, 9, 11, 9),
    (11, 9, 11, 7),
    (15, -1, 15, 11),
    (15, 11, 9, 11),
    (9, 11, 9, 3),
    (9, 3, 7, 3),
    (7, 3, 7, 5),
    (7, 5, 5, 5),
    (5, 5, 5, 3),
    (5, 3, -1, 3),
]

for x, y in csv.reader(open('coords.csv')):
    x_values.append(float(x)*1000)
    y_values.append(float(y)*1000)

fig = plt.figure(figsize=(10, 6))
plt.plot(x_values, y_values)

for x1, y1, x2, y2 in boundaries:
    plt.plot((x1*u, x2*u), (y1*u, y2*u), color='gray', linewidth=5)

plt.axis('equal')
plt.title("Level 1 Robot Position")
plt.xlabel("X (mm)")
plt.ylabel("Y (mm)")
plt.show()
fig.savefig('level1.png', dpi=500)
