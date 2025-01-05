const { expect } = require('chai');
const { execSync } = require('child_process');
const fs = require('fs/promises');
const path = require('path');
const os = require('os');

describe('CLI', () => {
    let tempDir;
    let originalCwd;

    beforeEach(async () => {
        // Save original working directory
        originalCwd = process.cwd();
        
        // Create a temporary directory for tests
        tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'create-cursor-app-'));
        
        // Create test templates directory
        const templatesDir = path.join(tempDir, 'templates/basic');
        await fs.mkdir(templatesDir, { recursive: true });
        
        // Create required template files
        await fs.writeFile(
            path.join(templatesDir, 'PROMPT.md'),
            '# {{project_name}}'
        );
        await fs.writeFile(
            path.join(templatesDir, 'CHANGELOG.md'),
            '# Changelog for {{project_name}}'
        );
        await fs.writeFile(
            path.join(templatesDir, 'PROJECT_SCOPE.md'),
            '# Scope for {{project_name}}'
        );
        await fs.writeFile(
            path.join(templatesDir, '.cursorrules'),
            JSON.stringify({
                settings: {
                    context_tracking: {
                        enabled: true,
                        files: ["PROMPT.md", "CHANGELOG.md", "PROJECT_SCOPE.md"]
                    }
                }
            })
        );

        // Change to temp directory for test execution
        process.chdir(tempDir);
    });

    afterEach(async () => {
        // Restore original working directory
        process.chdir(originalCwd);
        
        // Clean up temporary directory
        await fs.rm(tempDir, { recursive: true, force: true });
    });

    it('should create a new project with default template', async () => {
        const projectDir = 'test-project';
        
        // Run CLI with test mode
        process.env.TEST_MODE = '1';
        execSync(`node ${path.join(__dirname, '../bin/create-cursor-app.js')} ${projectDir}`);

        // Verify files were created
        const files = await fs.readdir(path.join(tempDir, projectDir));
        expect(files).to.include('PROMPT.md');
        expect(files).to.include('CHANGELOG.md');
        expect(files).to.include('PROJECT_SCOPE.md');

        // Verify content
        const content = await fs.readFile(path.join(tempDir, projectDir, 'PROMPT.md'), 'utf8');
        expect(content).to.equal('# Test Project');
    });

    it('should handle file overwrite confirmation', async () => {
        const projectDir = 'test-project';
        await fs.mkdir(projectDir, { recursive: true });
        
        // Create existing file
        await fs.writeFile(
            path.join(projectDir, 'PROMPT.md'),
            'existing content'
        );

        // Run CLI with test mode (which auto-confirms overwrites)
        process.env.TEST_MODE = '1';
        execSync(`node ${path.join(__dirname, '../bin/create-cursor-app.js')} ${projectDir}`);

        // Verify file was overwritten
        const content = await fs.readFile(path.join(projectDir, 'PROMPT.md'), 'utf8');
        expect(content).to.equal('# Test Project');
    });
}); 