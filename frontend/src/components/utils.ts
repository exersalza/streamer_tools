export const HOST = location.hostname;
export const PORT = 22727;

export const BACKEND = `http://${HOST}:${PORT}`;
export const API = `${BACKEND}/api/v1`;

export const RECONNECT_ATTEMPTS = 3;
export const RECONNECT_AFTER = 5; // in seconds

export function parseTime(input: number): String {
  const hours = Math.floor(input / 3600);
  const minutes = Math.floor((input % 3600) / 60);
  const remainingSeconds = input % 60;

  const formattedHours = String(hours).padStart(2, '0');
  const formattedMinutes = String(minutes).padStart(2, '0');
  const formattedSeconds = String(remainingSeconds).padStart(2, '0');

  return `${formattedHours}:${formattedMinutes}:${formattedSeconds}`;
}

export const _T = {
  legend: {
    titles: {
      updateTimes: "<update>"
    }
  }
}

export function stringToHexColor(str: string) {
  // Simple hash function
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
  }
  // Convert hash to 6-digit hex
  let color = "#";
  for (let i = 0; i < 3; i++) {
    const value = (hash >> (i * 8)) & 0xFF;
    color += value.toString(16).padStart(2, "0");
  }
  return color;
}
