import json
import math

from wpimath.geometry import Pose2d, Rotation2d, Translation2d
from wpimath.trajectory import TrajectoryGenerator, TrajectoryConfig
import matplotlib.pyplot as plt


trajectory = TrajectoryGenerator.generateTrajectory(
    Pose2d(0, 0, Rotation2d.fromDegrees(0)),
    [],
    # [Translation2d(1.5, 0.8)],
    Pose2d(3, 0, Rotation2d.fromDegrees(0)),
    TrajectoryConfig.fromFps(2.0, 0.5),
)

samples = 300
step = trajectory.totalTime() / samples
t = 0
points = []
last = None
while t <= trajectory.totalTime():
    sample = trajectory.sample(t)
    if last is None:
        last = sample
    omega = sample.curvature * sample.velocity
    dx = sample.pose.X() - last.pose.X()
    dy = sample.pose.Y() - last.pose.Y()
    theta = math.atan2(dy, dx)
    points.append((sample.pose.X(), sample.pose.Y(), theta, sample.velocity, omega))
    t += step

# Extracting x, y, and velocity from the points
x_values = [p[0] for p in points]
y_values = [p[1] for p in points]
velocity_values = [max(0.0, p[3]) for p in points]

# Plotting
plt.figure(figsize=(10, 6))
sc = plt.scatter(x_values, y_values, c=velocity_values, cmap='viridis', s=10)
plt.colorbar(sc, label='Velocity')
plt.axis('equal')
plt.xlabel("X")
plt.ylabel("Y")
plt.title(f"t={trajectory.totalTime():.2f} sec")
plt.show()

data = {
    "total_time": trajectory.totalTime(),
    "step_time": step,
    "points": points
}

json.dump(data, open("trajectory.json", "w"), separators=(',', ':'))
