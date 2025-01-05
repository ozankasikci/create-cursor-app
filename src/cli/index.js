const { Command } = require('commander');
const inquirer = require('inquirer');
const { join } = require('path');
const chalk = require('chalk');
const Template = require('../lib/templates');
const TemplateProcessor = require('../lib/templates/processor');

async function run() {
    console.log('\n' + chalk.cyan.bold('🚀 Create Cursor App'));
    console.log(chalk.dim('Interactive project generator\n'));

    const program = new Command()
        .version('0.1.0')
        .description('Creates new Cursor IDE projects from templates')
        .option('-t, --template <type>', 'template to use', 'basic')
        .argument('[directory]', 'directory to create the project in')
        .parse(process.argv);

    const options = program.opts();
    const [directory] = program.args;

    if (!directory) {
        console.error(chalk.red('Error: Directory argument is required'));
        process.exit(1);
    }

    // Get project name through interactive prompt
    const { projectName } = process.env.TEST_MODE ? 
        { projectName: 'Test Project' } :
        await inquirer.prompt([{
            type: 'input',
            name: 'projectName',
            message: "What's your project name?",
            default: 'my-awesome-project',
            validate: input => input.trim() !== '' || 'Project name is required'
        }]);

    console.log(chalk.dim('\n⚙️  Creating project...'));

    try {
        // Load templates
        const templates = await Template.loadAll();
        const template = templates.find(t => t.name === options.template);

        if (!template) {
            throw new Error(`Template '${options.template}' not found`);
        }

        // Create processor and process template
        const processor = new TemplateProcessor(
            template,
            join(process.cwd(), directory),
            projectName
        );

        await processor.process();

        console.log(`\n${chalk.green('✨')} Project ${chalk.cyan.bold(projectName)} created successfully!\n`);
        console.log(chalk.dim('To get started:'));
        console.log(`  cd ${chalk.cyan(directory)}`);
        console.log(`  ${chalk.green('Happy coding! 🎉')}\n`);

    } catch (error) {
        console.error(chalk.red('\nError:'), error.message);
        process.exit(1);
    }
}

module.exports = { run }; 