const fs = require('fs');
const path = require('path');

const lighthouseDir = './.lighthouseci';
const ALLOWED_METRICS = ['first-contentful-paint', 'largest-contentful-paint', 'first-meaningful-paint', 'speed-index', 'total-blocking-time', 'max-potential-fid', 'cumulative-layout-shift', 'server-response-time', 'interactive', 'mainthread-work-breakdown', 'bootup-time']

function getLatestLighthouseJson() {
  const files = fs.readdirSync(lighthouseDir);
  const matchingFiles = files.filter((file) => file.startsWith('lhr-') && file.endsWith('.json'));

  if (matchingFiles.length === 0) {
    console.error('No matching files found in the lighthouseci directory.');
    return null;
  }

  const sortedFiles = matchingFiles.sort((a, b) => {
    const timestampA = parseInt(a.match(/\d+/)[0]);
    const timestampB = parseInt(b.match(/\d+/)[0]);
    return timestampB - timestampA;
  });

  const latestFile = sortedFiles[0];
  return path.join(lighthouseDir, latestFile);
}

function generateReadme(metrics) {
  let readme = '# Performance Metrics\n\n';

  for (const key of ALLOWED_METRICS) {
    const metric = metrics[key];
  
    readme += `## ${metric.title}\n\n`;
    readme += `${metric.description}\n\n`;
    readme += `**Score:** ${metric.score} (${metric.scoreDisplayMode})\n`;
    readme += `**Value:** ${metric.displayValue} (${metric.numericValue} ${metric.numericUnit})\n\n`;
  }

  return readme;
}

function updateReadme(content) {
  try {
    const existingReadme = fs.readFileSync('README.md', 'utf-8');
    const startIndex = existingReadme.indexOf('# Performance Metrics');
    if (startIndex !== -1) {
      const updatedReadme = existingReadme.slice(0, startIndex) + content;

      fs.writeFile('README.md', updatedReadme, (err) => {
        if (err) throw err;
        console.log('README.md file has been created successfully.');
      });
    } else {
      console.error('Could not find the "# Performance Metrics" line in README.md.');
    }
  } catch (error) {
    console.error('Error reading or updating README.md:', error);
  }
}

const latestJsonFile = getLatestLighthouseJson();

if (latestJsonFile) {
  const rawData = fs.readFileSync(latestJsonFile);
  const jsonData = JSON.parse(rawData);
  const readmeContent = generateReadme(jsonData.audits);

  updateReadme(readmeContent)
} else {
  console.error('Failed to find a matching JSON file.');
}
