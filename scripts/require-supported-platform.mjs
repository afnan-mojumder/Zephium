if (process.platform !== "darwin" && process.platform !== "win32" && process.platform !== "linux") {
  console.error("Zephium is not supported on this platform.");
  process.exit(1);
}
