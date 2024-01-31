# Y.A.c.H.T. - Yet Another CLI Habit Tracker written in Rust

### About

Y.A.C.H.T., a command-line habit tracker written in Rust. This tool allows you to track the activities that make up your perfect day and monitor your daily progress effortlessly.

### Features

- **Text User Interface (TUI)**
- **Effortless Activities Tracking**
- **Progress Monitoring**

### Demo

![yacht_demo](https://github.com/tracyspacy/yacht/assets/42025315/68513987-7f2a-4474-868f-165b40220565)

gif was created using [VHS](https://github.com/charmbracelet/vhs)

### Installation

```
git clone https://github.com/tracyspacy/yacht.git
cd yacht
cargo build --release
```

### Usage

- **Adding Activities**: Press `n` to add a new activity. Specify the frequency using `AW` (all week), `WD` (working days), or `WE` (weekends).
- **Navigating Activities**: Use the `UP` and `DOWN` arrow keys to select a specific activity.
- **Marking Activities Done**: Press `d` to mark the selected activity as done for the day.
- **Removing Activities**: Press `r` to remove the selected activity.
- **Quitting the Program**: Press `q` to exit the program.

### To-Do

- **Expand Frequency Types**: Add more frequency types besides the existing ones (week days basically) to provide users with greater flexibility in scheduling their activities.
- **Stats Mode**: Implement a stats mode to allow users to view statistics by activity, such as how often an activity was done over a specified period.

### Contributing

Contributions are welcome! If you encounter any bugs, have suggestions for improvements, or would like to contribute new features, please open an issue or submit a pull request.

