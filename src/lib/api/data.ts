import { invoke } from "@tauri-apps/api/core";

import type { FileType } from "./types";

/**
 * Upload a file to the backend file API
 *
 * @param fileType Type of file (Determines the file path)
 * @param file The file to upload
 * @returns URL pointing to the uploaded file
 */
export async function uploadFile(
  fileType: FileType,
  file: File
): Promise<string> {
  const fileName = file.name;
  const fileData = await file.arrayBuffer();

  return invoke<string>("upload_file", {
    fileType,
    fileName,
    fileData,
  });
}
