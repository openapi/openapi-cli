
la cli openapi implementa le seguenti chiamate


o "Usage:  [COMMAND] [OPTIONS]"
echo ""
echo "Available options"
echo "  -S, --sandbox  Execute in the sandbox environment"
echo "  -V, --version  Print version info and exit"
echo "  -h, --help     Print help information"
echo ""

Esempi di utilizzo:
openapi token create --scopes "GET:test.imprese.openapi.it/advance" 
openapi sms send --to "+391234567890" --message "Hello from Openapi CLI!"
openapi geocoding search --query "Piazza del Colosseo, Rome" --limit 5

variabili supportate
OPENAPI_USERNAME
OPENAPI_KEY
OPENAPI_SANDBOX_KEY




qui una lista di tutte le api che la cli implementa

Oauth
The key to secure access to all APIs.

Company
All the data about companies in a single API and in just a few seconds


SMS v2
High-Quality SMS with a custom sender, real-time delivery reports, and instant sending.


eSignature
eSignature solutions with European validity, all available in one API


Risk
All reports and data to verify the reliability of private and legal entities


AI
Integrate the power of language models with your company’s data to deliver accurate and contextualized answers.


Trust
Trust is the best solution to prevent malicious actions on your platform through APIs.


Geocoding
From street addresses to geographical coordinates (and vice versa) in just a second

Invoice
Send electronic receipts and invoices to the Revenue Agency quickly, easily, and securely


Automotive
All the latest information on cars, motorbikes, insurance available in real time and globally.


DocuEngine
All official documents from the Business Register, Revenue Agency, Municipalities, and INPS

Chamber of Commerce Company Registrations
The documents of the Italian Chamber of Commerce in seconds and without search costs


Real Estate
The fundamental data for any effective real estate valuation!


Zip Codes
All updated data on zip codes, municipalities, provinces, regions via API

Visengine
All official documents from Chamber of Commerce, INPS and Tax Agency

Italian cadastre
All cadastral data and property owners in Italy

Postal Service
All your mailings via API and in real time

Massive REM
The easiest and most secure solution for sending your Massive REMs


PDF
The only API to transform HTML content into PDF, which also renders JavaScript code

Time Stamping
Date and time stamp any document

Italian certified e-mail
Legalmail CEM for sending communications and documents securely and with full legal value

Paying Bills
Bills payment via API and in real time

Exchange Rate
Check foreign currency exchange rates worldwide

.it Domains
The only API to activate and manage .it domains in real time

SMS v1
API SMS v1 has been replaced by the endpoints available in SMS v2.

SDI Electronic Invoicing
Electronic invoicing easy, fast and with high security standards


Enterprises
The API has been replaced by the new endpoints available in Company


Digital Signature
The API has been replaced by the endpoints available in eSignature
