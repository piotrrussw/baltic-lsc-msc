import fs from 'fs';
import path from 'path';

const lighthouseDir = './.lighthouseci';

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

  for (const key in metrics) {
    const metric = metrics[key];
    readme += `## ${metric.title}\n\n`;
    readme += `${metric.description}\n\n`;
    readme += `**Score:** ${metric.score} (${metric.scoreDisplayMode})\n`;
    readme += `**Value:** ${metric.displayValue} (${metric.numericValue} ${metric.numericUnit})\n\n`;
  }

  return readme;
}

const latestJsonFile = getLatestLighthouseJson();

if (latestJsonFile) {
  const rawData = fs.readFileSync(latestJsonFile);
  const jsonData = JSON.parse(rawData);

  const readmeContent = generateReadme(jsonData.audits);

  console.log('Loaded JSON data from the latest file:');

  fs.writeFile('README.md', readmeContent, (err) => {
    if (err) throw err;
    console.log('README.md file has been created successfully.');
  });
} else {
  console.error('Failed to find a matching JSON file.');
}


