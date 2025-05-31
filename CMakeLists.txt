# CMakeLists.txt (optional, for systemd integration)
cmake_minimum_required(VERSION 3.16)
project(hyprmenu-rs)

# Install desktop file and systemd service if needed
install(FILES hyprmenu.desktop DESTINATION share/applications)
