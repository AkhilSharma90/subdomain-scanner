function resetResults() {
    resultsSection = document.getElementById('results-section');
    resultsSection.innerHTML = '';
}

const scanButton = document.getElementById('scan-button');
scanButton.addEventListener("click", function() {
    resetResults();
    console.log("Scan begun!");

    let inputField = document.getElementById("query-input");
    let domain = inputField.value;
    
    const resultsSection = document.getElementById('results-section');

    // Create a header to display a 'scanning' message
    const heading = document.createElement('h3');
    heading.setAttribute('id', 'fetch-results-heading');
    resultsSection.appendChild(heading);
    heading.innerText = `Scanning ${domain}`;

    let url = `api/v1/scan/${domain}`;
    let call = apiCall(domain, url, "GET");
    call.send();

    document.getElementById("query-input").value = null;
});

function exportJson(rawData) {
    console.log("Exporting...");
    console.log("Raw json: ", rawData);

    const jsonString = "data:text/json;charset=utf-8," + 
        encodeURIComponent(JSON.stringify(rawData));

    const download = document.createElement('a');
    download.setAttribute('href', jsonString);
    download.setAttribute('download', "results.json");
    download.click();
    download.remove();
}

function renderNotFound(query) {
    console.log("Not found!");
    const resultsSection = document.getElementById('results-section');

    const previousHeading = document.getElementById('fetch-results-heading');
    if (previousHeading) previousHeading.remove();

    const heading = document.createElement('h3');
    heading.setAttribute('id', 'show-results-heading');
    resultsSection.appendChild(heading);
    heading.innerText = `No results found for ${query}`;
}

function renderResults(query, results) {
    if (results.length == 0) {
        renderNotFound(query);
        return;
    }
    console.log("started rendering results");
    const resultsSection = document.getElementById('results-section');

    const previousHeading = document.getElementById('fetch-results-heading');
    if (previousHeading) previousHeading.remove();

    const heading = document.createElement('h3');
    heading.setAttribute('id', 'show-results-heading');
    resultsSection.appendChild(heading);
    heading.innerText = `Showing results for ${query}`;

    const resultsContainer = document.createElement('div');
    resultsContainer.setAttribute("id", "results-container");
    resultsSection.appendChild(resultsContainer);

    const exportButtonDiv = document.createElement('div');
    exportButtonDiv.setAttribute('id', 'export-button-container');
    resultsContainer.appendChild(exportButtonDiv);

    const exportButton = document.createElement('button');
    exportButton.setAttribute('id', 'export-results');
    exportButtonDiv.appendChild(exportButton);
    exportButton.innerHTML = '<span class="export-text">Export Results</span><span class="fa-solid fa-download"></span>'
    
    exportButton.addEventListener('click', function() {
        console.log("exporting json...");
        exportJson(results);
    });

    for(const result of results) {
        console.log("scanning ", result["domain_name"]);
        let newResult = document.createElement('div');
        newResult.classList.add("result");
        resultsContainer.appendChild(newResult);

        let subdomainRow = document.createElement('div');
        subdomainRow.classList.add("subdomain-row");
        newResult.appendChild(subdomainRow);

        let subdomain = document.createElement('div');
        subdomain.classList.add("subdomain");
        subdomainRow.appendChild(subdomain);
        subdomain.innerText = `${result["domain_name"]}`;

        let showButton = document.createElement('button');
        showButton.classList.add("show-ports-btn");
        subdomainRow.appendChild(showButton);
        showButton.innerText = `Ports`;

        showButton.addEventListener("click", function() {
            console.log("show button clicked!");
            let container = this.parentElement.nextElementSibling;
            this.parentElement.nextElementSibling.classList.toggle("show-ports");
        });

        let portsContainer = document.createElement('div');
        portsContainer.classList.add("ports-container");
        newResult.appendChild(portsContainer);
        
        let openPorts = result["open_ports"];
        console.log("openPorts: ", openPorts);

        for(i = 0; i < openPorts.length; ++i) {
            console.log("Handling: ", openPorts[i]);
            let port = document.createElement('div');
            port.classList.add("port");

            let portNumber = openPorts[i]["port"];
            let portNumberDiv = document.createElement('div');
            portNumberDiv.classList.add("port-number");
            portNumberDiv.innerText = portNumber;

            let openStatus = openPorts[i]["conn_open"];
            let openStatusDiv = document.createElement('div');
            openStatusDiv.classList.add("open-status");
            if(openStatus === true) {
                openStatusDiv.innerHTML = `<i class="fa-solid fa-circle open-port"></i>`;
            } else {
                openStatusDiv.innerHTML = `<i class="fa-solid fa-circle closed-port"></i>`;
            }        

            port.appendChild(portNumberDiv);
            port.appendChild(openStatusDiv);

            portsContainer.appendChild(port);
        }
    }
    console.log("rendering done; document: ", document.documentElement);
}


function apiCall(query, url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function() {
        if (this.readyState === this.DONE) {
            const results = JSON.parse(this.responseText)["subdomains"];
            console.log("subdomains: ", results);
            renderResults(query, results);
        }
    });
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', 'token');
    return xhr;
}