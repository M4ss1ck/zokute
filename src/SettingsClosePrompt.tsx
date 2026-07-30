interface Props {
  onSave: () => void;
  onDiscard: () => void;
  onKeepEditing: () => void;
}

export function SettingsClosePrompt({ onSave, onDiscard, onKeepEditing }: Props) {
  return (
    <div className="settingsPromptScrim">
      <div
        className="settingsPrompt"
        role="dialog"
        aria-modal="true"
        aria-labelledby="settings-prompt-title"
        tabIndex={-1}
        onKeyDown={(event) => {
          if (event.key === "Escape") onKeepEditing();
        }}
      >
        <h2 id="settings-prompt-title">Unsaved changes</h2>
        <p>Keep the changes you made, or drop them and go back to the last saved setup?</p>
        <div className="settingsPromptActions">
          <button type="button" className="settingsButton" onClick={onKeepEditing}>Keep editing</button>
          <button type="button" className="settingsButton" onClick={onDiscard}>Discard and close</button>
          <button type="button" className="settingsButton settingsButtonPrimary" onClick={onSave}>Save and close</button>
        </div>
      </div>
    </div>
  );
}
