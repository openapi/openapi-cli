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

    /// Discover who you are
    #[arg(long, global = true)]
    pub who: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// OAuth token management
    #[command(name = "oauth")]
    Token {
        #[command(subcommand)]
        command: token::TokenCommands,
    },

    /// OAuth v2 token management and analytics
    #[command(name = "oauthv2")]
    Oauthv2 {
        #[command(subcommand)]
        command: oauthv2::Oauthv2Commands,
    },

    /// Electronic signature
    Esignature {
        #[command(subcommand)]
        command: esignature::EsignatureCommands,
    },

    /// AI language models
    Ai {
        #[command(subcommand)]
        command: ai::AiCommands,
    },

    /// SMS messaging (v2)
    #[command(name = "smsv2")]
    Sms {
        #[command(subcommand)]
        command: sms::SmsCommands,
    },

    /// Trust verification services
    Trust {
        #[command(subcommand)]
        command: trust::TrustCommands,
    },

    /// Foreign currency exchange rates
    #[command(name = "exchange")]
    ExchangeRate {
        #[command(subcommand)]
        command: exchange_rate::ExchangeRateCommands,
    },

    /// Risk reports and scoring
    Risk {
        #[command(subcommand)]
        command: risk::RiskCommands,
    },

    /// Automotive data (vehicles, insurance)
    Automotive {
        #[command(subcommand)]
        command: automotive::AutomotiveCommands,
    },

    /// SDI electronic invoicing
    Sdi {
        #[command(subcommand)]
        command: sdi::SdiCommands,
    },

    /// Document time stamping
    #[command(name = "marchetemporali")]
    TimeStamping {
        #[command(subcommand)]
        command: time_stamping::TimeStampingCommands,
    },

    /// Real estate valuation data
    #[command(name = "realestate")]
    RealEstate {
        #[command(subcommand)]
        command: real_estate::RealEstateCommands,
    },

    /// Italian cadastral data
    #[command(name = "catasto")]
    Cadastre {
        #[command(subcommand)]
        command: cadastre::CadastreCommands,
    },

    /// Italian certified email (PEC / Legalmail)
    #[command(name = "pec")]
    CertifiedEmail {
        #[command(subcommand)]
        command: certified_email::CertifiedEmailCommands,
    },

    /// .it domain management
    Domains {
        #[command(subcommand)]
        command: domains::DomainsCommands,
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

    /// Massive Registered Electronic Mail
    #[command(name = "pecmassiva")]
    MassiveRem {
        #[command(subcommand)]
        command: massive_rem::MassiveRemCommands,
    },

    /// Bills payment
    #[command(name = "bollettini")]
    PayingBills {
        #[command(subcommand)]
        command: paying_bills::PayingBillsCommands,
    },

    /// HTML to PDF conversion
    Pdf {
        #[command(subcommand)]
        command: pdf::PdfCommands,
    },

    /// Postal mail service
    #[command(name = "ufficiopostale")]
    PostalService {
        #[command(subcommand)]
        command: postal_service::PostalServiceCommands,
    },

    /// Official documents (Chamber of Commerce, INPS, Tax Agency)
    Visengine {
        #[command(subcommand)]
        command: visengine::VisengineCommands,
    },

    /// Zip codes, municipalities, provinces, regions
    #[command(name = "cap")]
    ZipCodes {
        #[command(subcommand)]
        command: zip_codes::ZipCodesCommands,
    },

    /// Company data and information
    Company {
        #[command(subcommand)]
        command: company::CompanyCommands,
    },

    /// Chamber of Commerce documents
    #[command(name = "visurecamerali")]
    ChamberOfCommerce {
        #[command(subcommand)]
        command: chamber_of_commerce::ChamberOfCommerceCommands,
    },

    /// Official documents (Business Register, Revenue Agency, INPS)
    Docuengine {
        #[command(subcommand)]
        command: docuengine::DocuengineCommands,
    },

    /// Show configuration status and readiness
    Info,
}
