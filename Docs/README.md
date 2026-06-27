# Maxx Steele documentation

Community manuals for owners, programmers, and repairers. Each manual has markdown chapters and a rebuilt PDF.

| Manual | Path | PDF | Build |
|--------|------|-----|-------|
| **User** | [`User/`](User/) | [`Maxx-Steele-User-Manual.pdf`](User/Maxx-Steele-User-Manual.pdf) | `python3 tools/build_user_manual_pdf.py` |
| **Technical** | [`Technical/`](Technical/) | [`Maxx-Steele-Technical-Manual.pdf`](Technical/Maxx-Steele-Technical-Manual.pdf) | `python3 tools/build_technical_manual_pdf.py` |
| **Mechanical** | [`Mechanical/`](Mechanical/) | [`Maxx-Steele-Mechanical-Manual.pdf`](Mechanical/Maxx-Steele-Mechanical-Manual.pdf) | `python3 tools/build_mechanical_manual_pdf.py` |

Factory archival scans remain under [`Chassis/Manual/`](../Chassis/Manual/).

GitHub Actions rebuilds each PDF on push when the corresponding `Docs/*` tree changes.