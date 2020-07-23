import * as vscode from 'vscode';

export async function createWithPapProxyLaunchConfiguration() {
    if (!vscode.workspace.workspaceFolders) {
        throw new Error("No workspace opened.");
    }
    const configs: any[] = vscode.workspace.getConfiguration('launch', vscode.workspace.workspaceFolders[0].uri).configurations;
    if (configs.length === 0) {
        vscode.window.showErrorMessage("No launch configurations found.");
    }
    const result = await vscode.window.showQuickPick(configs.map(c => c.name));
    const config = configs.find(c => c.name === result);
    
    // wip
    vscode.window.showInformationMessage("Lauch configuration was created.");
}
