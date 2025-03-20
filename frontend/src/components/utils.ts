export const HOST = location.hostname;
export const PORT = 22727;

export const BACKEND = `http://${HOST}:${PORT}`;
export const API = `${BACKEND}/api/v1`;


export function parseTime(input: number): String {
  const hours = Math.floor(input / 3600);
  const minutes = Math.floor((input % 3600) / 60);
  const remainingSeconds = input % 60;
  
  const formattedHours = String(hours).padStart(2, '0');
  const formattedMinutes = String(minutes).padStart(2, '0');
  const formattedSeconds = String(remainingSeconds).padStart(2, '0');
  
  return `${formattedHours}:${formattedMinutes}:${formattedSeconds}`;
}
