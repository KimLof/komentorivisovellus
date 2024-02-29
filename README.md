# Tiedostojen Lajittelusovellus

Tämä on yksinkertainen komentorivisovellus, jonka avulla voit lajitella tiedostoja eri kansioihin niiden tiedostotyypin perusteella. Sovellus tukee kuvien, äänitiedostojen, videoiden, asiakirjojen ja arkistojen lajittelua.

## Käyttöohjeet

1. **Asennus**: Voit ladata sovelluksen GitHubista ja asentaa sen omalle tietokoneellesi seuraamalla näitä ohjeita:
    - Kloonaa GitHub-repositorio omalle tietokoneellesi: `git clone https://github.com/kayttaja/tiedostojen-lajittelusovellus.git`
    - Siirry sovelluskansion sisälle: `cd tiedostojen-lajittelusovellus`
    - Käännä sovellus: `cargo build --release`
    - Sovelluksen suorittaminen: `cargo run --release`

2. **Käynnistäminen**: Kun sovellus on asennettu, voit käynnistää sen komentoriviltä seuraavasti:
    - Avaa komentorivi ja siirry sovelluskansion sisälle.
    - Käynnistä sovellus komennolla: `cargo run --release`

3. **Käyttö**:
    - Anna ensin polku hakemistoon, jonka haluat lajitella.
    - Valitse tiedostotyyppi, jonka haluat lajitella. Voit valita yhden tai useamman vaihtoehdon.
    - Valitse, haluatko käsitellä alihakemistoja.
    - Voit valita, haluatko nähdä esikatselun löydetyistä tiedostoista.
    - Paina "Siirrä Tiedostot" -painiketta aloittaaksesi lajittelun.

4. **Virheenkäsittely**: Jos sovellus kohtaa virheitä, se ilmoittaa niistä komentorivillä ja antaa ohjeita ongelman korjaamiseksi.

5. **Palautteen Antaminen**: Jos kohtaat ongelmia tai sinulla on ehdotuksia sovelluksen parantamiseksi, voit jakaa palautteesi GitHubissa avaamalla uuden [ongelman](https://github.com/KimLof/tiedostosovellus/issues) tai tekemällä lähettämällä sähköpostia osoitteeseen kim@kimcode.fi.

## Toiminnallisuudet
- Mahdollisuus lajitella tiedostoja eri kansioihin niiden tiedostotyypin perusteella.
- Tuki kuvien, äänitiedostojen, videoiden, asiakirjojen ja arkistojen lajittelulle.
- Käsittele alihakemistoja tarvittaessa.
- Näytä esikatselu löydetyistä tiedostoista ennen lajittelun aloittamista.

## TODO 
- Alihakemiston käsittely ei toimi kunnolla.
- Kansiot luodaan vain, jos löydetään tiedostoja.
- Peruutus- ja palautustoiminto.
- Kielen vaihto.
- Esikatselu löydetyistä tiedostoista.

### HUOM: SOVELLUS EI OLE VALMIS VIELÄ ###

Tätä README.md-tiedostoa päivitetään jatkuvasti, kun sovellusta kehitetään edelleen. Lisää toiminnallisuuksia ja parannuksia on tulossa tulevissa versioissa. Jos sinulla on kysyttävää tai tarvitset lisätietoja, älä epäröi ottaa yhteyttä!
