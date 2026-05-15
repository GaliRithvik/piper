// Piper VS Code Extension
// Provides: syntax highlighting, run-in-terminal, piper fmt on save

const vscode = require('vscode');
const path   = require('path');
const cp     = require('child_process');

let piperTerminal = null;

function getPiperPath() {
  return vscode.workspace.getConfiguration('piper').get('executablePath') || 'piper';
}

function getOrCreateTerminal() {
  const reuse = vscode.workspace.getConfiguration('piper').get('reuseTerminal', true);
  if (reuse && piperTerminal && !piperTerminal.exitStatus) {
    return piperTerminal;
  }
  piperTerminal = vscode.window.createTerminal('Piper');
  return piperTerminal;
}

// ── Run File ────────────────────────────────────────────────────────────────

async function runFile() {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    vscode.window.showErrorMessage('Piper: No active editor.');
    return;
  }
  if (editor.document.languageId !== 'piper' &&
      !editor.document.fileName.endsWith('.piper')) {
    vscode.window.showErrorMessage('Piper: Active file is not a .piper file.');
    return;
  }

  // Save the file first
  if (editor.document.isDirty) {
    await editor.document.save();
  }

  const filePath = editor.document.fileName;
  const piperExe = getPiperPath();
  const term     = getOrCreateTerminal();

  term.show(true);
  term.sendText(`${piperExe} "${filePath}"`);
}

// ── Format File ─────────────────────────────────────────────────────────────

async function formatFile(document) {
  const doc = document || (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document);
  if (!doc) {
    vscode.window.showErrorMessage('Piper: No active editor.');
    return;
  }
  if (!doc.fileName.endsWith('.piper')) {
    vscode.window.showErrorMessage('Piper: Active file is not a .piper file.');
    return;
  }

  // Save first
  if (doc.isDirty) {
    await doc.save();
  }

  const filePath = doc.fileName;
  const piperExe = getPiperPath();

  return new Promise((resolve) => {
    cp.exec(`"${piperExe}" fmt "${filePath}"`, (err, stdout, stderr) => {
      if (err) {
        vscode.window.showErrorMessage(`piper fmt failed: ${stderr || err.message}`);
      } else {
        // File was written in place — VS Code detects the change automatically
        const msg = stdout.trim();
        if (msg) vscode.window.setStatusBarMessage(`$(check) ${msg}`, 3000);
      }
      resolve();
    });
  });
}

// ── Document Formatter provider ──────────────────────────────────────────────
// Lets VS Code's built-in "Format Document" (Shift+Alt+F) trigger `piper fmt`.

class PiperFormatter {
  provideDocumentFormattingEdits(document) {
    const piperExe = getPiperPath();
    const src      = document.getText();

    return new Promise((resolve) => {
      const proc = cp.exec(`"${piperExe}" fmt --stdout`, (err, stdout, stderr) => {
        if (err || !stdout) {
          // Fallback: run fmt on the file itself
          formatFile(document).then(() => resolve([]));
          return;
        }
        const fullRange = new vscode.Range(
          document.positionAt(0),
          document.positionAt(src.length)
        );
        resolve([vscode.TextEdit.replace(fullRange, stdout)]);
      });
      proc.stdin.write(src);
      proc.stdin.end();
    });
  }
}

// ── Activation ───────────────────────────────────────────────────────────────

function activate(context) {
  // Commands
  context.subscriptions.push(
    vscode.commands.registerCommand('piper.runFile',    runFile),
    vscode.commands.registerCommand('piper.formatFile', () => formatFile()),
  );

  // Document formatter (Shift+Alt+F / Format Document)
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider(
      { language: 'piper' },
      new PiperFormatter()
    )
  );

  // Format on save
  context.subscriptions.push(
    vscode.workspace.onWillSaveTextDocument(async (e) => {
      if (!e.document.fileName.endsWith('.piper')) return;
      const formatOnSave = vscode.workspace.getConfiguration('piper').get('formatOnSave', false);
      if (!formatOnSave) return;
      e.waitUntil(formatFile(e.document));
    })
  );

  // Clean up terminal on close
  context.subscriptions.push(
    vscode.window.onDidCloseTerminal((t) => {
      if (t === piperTerminal) piperTerminal = null;
    })
  );

  console.log('Piper extension activated.');
}

function deactivate() {}

module.exports = { activate, deactivate };
