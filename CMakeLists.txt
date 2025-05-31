# CMakeLists.txt (optional, for systemd integration)
cmake_minimum_required(VERSION 3.16)
project(quickmenu)

# Install desktop file and systemd service if needed
install(FILES quickmenu.desktop DESTINATION share/applications)
