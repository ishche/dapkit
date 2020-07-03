import * as vscode from 'vscode';
import { createWithPapProxyLaunchConfiguration } from "./commands";

export function activate(context: vscode.ExtensionContext) {
    vscode.commands.registerCommand("x0a-dapkit.create-with-dap-proxy-launch-configuration", createWithPapProxyLaunchConfiguration);
}

export function deactivate() { }
