

pub const input: &str = r##"
<OFX>
    <SIGNONMSGSRSV1>
        <SONRS>
            <STATUS>
                <CODE>0
                <SEVERITY>INFO
            </STATUS>
            <DTSERVER>20220212002229.121
            <LANGUAGE>ENG
            <FI>
                <ORG>bogus not real credit union
                <FID>4242
            </FI>
        </SONRS>
    </SIGNONMSGSRSV1>

    <BANKMSGSRSV1>
        <STMTTRNRS>
            <TRNUID>1
            <STATUS>
                <CODE>0
                <SEVERITY>INFO
            </STATUS>
            <STMTRS>
                <CURDEF>USD
                <BANKACCTFROM>
                    <BANKID>USA
                    <ACCTID>9999999999999999
                    <ACCTTYPE>CREDITLINE
                </BANKACCTFROM>

                <BANKTRANLIST>
                    <DTSTART>20200101
                    <DTEND>20220228
                    <STMTTRN>
                        <TRNTYPE>DEBIT
                        <DTPOSTED>20220227000000.000[-08:PST]
                        <TRNAMT>-45.60

                        <FITID>00000000 000000 0,000 0,000,000,000,000
                        <NAME>Ext Credit Card Debit A FOOD PLACE
                        <MEMO>Ext Credit Card Debit A FOOD PLACE              CITYNAME  XX
                    </STMTTRN>
                    <STMTTRN>
                        <TRNTYPE>DEBIT
                        <DTPOSTED>20220227000000.000[-08:PST]
                        <TRNAMT>-49.68

                        <FITID>11111111 111111 1,111 1,111,111,111,111
                        <NAME>Ext Credit Card Debit GROCERYSTORE 6969
                        <MEMO>Ext Credit Card Debit GROCERYSTORE 6969            CITYNAME     XX
                    </STMTTRN>
                    <STMTTRN>
                        <TRNTYPE>DEBIT
                        <DTPOSTED>20220227000000.000[-08:PST]
                        <TRNAMT>-10.86

                        <FITID>22222222 222222 2,222 2,222,222,222,222
                        <NAME>Ext Credit Card Debit SUBSCRIPTION SERVICE
                        <MEMO>Ext Credit Card Debit SUBSCRIPTION SERVICE   YEP
                    </STMTTRN>
                </BANKTRANLIST>
                <LEDGERBAL>
                <BALAMT>55555.55</BALAMT>
                <DTASOF>20220212002229.121</DTASOF>
                </LEDGERBAL>

                <AVAILBAL>
                <BALAMT>55555.55</BALAMT>
                <DTASOF>20220212002229.121</DTASOF>
                </AVAILBAL>

            </STMTRS>
        </STMTTRNRS>
    </BANKMSGSRSV1>
</OFX>

"##;
