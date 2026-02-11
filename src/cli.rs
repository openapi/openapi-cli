use clap::{Parser, Subcommand};

use crate::commands::*;

#[derive(Debug, Parser)]
#[command(
    name = "openapi",
    about = "CLI client for Openapi.com APIs",
    version,
    propagate_version = true
)]
pub struct Cli {
    /// Execute in the sandbox environment
    #[arg(short = 'S', long, global = true)]
    pub sandbox: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// OAuth token management
    Token {
        #[command(subcommand)]
        command: token::TokenCommands,
    },

    /// Company data and information
    Company {
        #[command(subcommand)]
        command: company::CompanyCommands,
    },

    /// SMS messaging (v2)
    Sms {
        #[command(subcommand)]
        command: sms::SmsCommands,
    },

    /// Electronic signature
    Esignature {
        #[command(subcommand)]
        command: esignature::EsignatureCommands,
    },

    /// Risk reports and scoring
    Risk {
        #[command(subcommand)]
        command: risk::RiskCommands,
    },

    /// AI language models
    Ai {
        #[command(subcommand)]
        command: ai::AiCommands,
    },

    /// Trust verification services
    Trust {
        #[command(subcommand)]
        command: trust::TrustCommands,
    },

    /// Geocoding and reverse geocoding
    Geocoding {
        #[command(subcommand)]
        command: geocoding::GeocodingCommands,
    },

    /// Electronic invoicing
    Invoice {
        #[command(subcommand)]
        command: invoice::InvoiceCommands,
    },

    /// Automotive data (vehicles, insurance)
    Automotive {
        #[command(subcommand)]
        command: automotive::AutomotiveCommands,
    },

    /// Official documents (Business Register, Revenue Agency, INPS)
    Docuengine {
        #[command(subcommand)]
        command: docuengine::DocuengineCommands,
    },

    /// Chamber of Commerce documents
    #[command(name = "chamber-of-commerce")]
    ChamberOfCommerce {
        #[command(subcommand)]
        command: chamber_of_commerce::ChamberOfCommerceCommands,
    },

    /// Real estate valuation data
    #[command(name = "real-estate")]
    RealEstate {
        #[command(subcommand)]
        command: real_estate::RealEstateCommands,
    },

    /// Zip codes, municipalities, provinces, regions
    #[command(name = "zip-codes")]
    ZipCodes {
        #[command(subcommand)]
        command: zip_codes::ZipCodesCommands,
    },

    /// Official documents (Chamber of Commerce, INPS, Tax Agency)
    Visengine {
        #[command(subcommand)]
        command: visengine::VisengineCommands,
    },

    /// Italian cadastral data
    Cadastre {
        #[command(subcommand)]
        command: cadastre::CadastreCommands,
    },

    /// Postal mail service
    #[command(name = "postal-service")]
    PostalService {
        #[command(subcommand)]
        command: postal_service::PostalServiceCommands,
    },

    /// Massive Registered Electronic Mail
    #[command(name = "massive-rem")]
    MassiveRem {
        #[command(subcommand)]
        command: massive_rem::MassiveRemCommands,
    },

    /// HTML to PDF conversion
    Pdf {
        #[command(subcommand)]
        command: pdf::PdfCommands,
    },

    /// Document time stamping
    #[command(name = "time-stamping")]
    TimeStamping {
        #[command(subcommand)]
        command: time_stamping::TimeStampingCommands,
    },

    /// Italian certified email (PEC / Legalmail)
    #[command(name = "certified-email")]
    CertifiedEmail {
        #[command(subcommand)]
        command: certified_email::CertifiedEmailCommands,
    },

    /// Bills payment
    #[command(name = "paying-bills")]
    PayingBills {
        #[command(subcommand)]
        command: paying_bills::PayingBillsCommands,
    },

    /// Foreign currency exchange rates
    #[command(name = "exchange-rate")]
    ExchangeRate {
        #[command(subcommand)]
        command: exchange_rate::ExchangeRateCommands,
    },

    /// .it domain management
    Domains {
        #[command(subcommand)]
        command: domains::DomainsCommands,
    },

    /// SDI electronic invoicing
    Sdi {
        #[command(subcommand)]
        command: sdi::SdiCommands,
    },

    /// Show configuration status and readiness
    Info,
}
