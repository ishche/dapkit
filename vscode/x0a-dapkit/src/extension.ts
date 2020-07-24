import * as vscode from 'vscode';
import { getLaunchLonfigurationForDapkit } from "./commands";

export function activate(context: vscode.ExtensionContext) {
    vscode.commands.registerCommand("x0a-dapkit.get-launch-configuration-for-dapkit", getLaunchLonfigurationForDapkit);
}

export function deactivate() { }
