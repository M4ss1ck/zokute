import { invoke } from "@tauri-apps/api/core";

interface Props {
  action: string;
  uri?: string;
  label: string;
  onAction?: () => void;
}

export function ActionTarget({ action, uri, label, onAction }: Props) {
  async function handleClick() {
    if (action === "open-uri" && uri) {
      await invoke("open_uri", { uri }).catch(() => {});
    } else if (action === "copy-visible-field") {
      await invoke("copy_text", { text: label }).catch(() => {});
    } else if (action === "refresh") {
      await invoke("refresh_plugin", {}).catch(() => {});
    }
    onAction?.();
  }

  return (
    <span
      className="actionTarget"
      role="button"
      tabIndex={0}
      title={uri ?? label}
      onClick={handleClick}
      onKeyDown={(e) => { if (e.key === "Enter") handleClick(); }}
    >
      {label}
    </span>
  );
}
