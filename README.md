Port 3030

http://<your-server-ip>:3030/update?domainname=subdomain.example.com&password=<cloudflare_api_token>&myip=<ipaddr>

	4.	Benutze eine Custom Template (Benutzerdefinierte Vorlage):
	•	Wähle die Option Create Custom Token (Benutzerdefinierten Token erstellen). So kannst du genau festlegen, welche Berechtigungen der Token haben soll.
	5.	Berechtigungen festlegen:
	•	In der Sektion Permissions (Berechtigungen) klickst du auf +Add more (Weitere hinzufügen).
	•	DNS:
	•	Permission: Wähle Edit (Bearbeiten).
	•	Zone: Wähle Specific Zone (Bestimmte Zone).
	•	Zone auswählen: Wähle die Zone (Domain) aus, die du aktualisieren möchtest (z.B. example.com).
Diese Berechtigung erlaubt dem API-Token, DNS-Einträge zu bearbeiten (was zum Aktualisieren des A-Records notwendig ist).