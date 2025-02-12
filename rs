<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QR Code Generator</title>
    <script src="https://cdn.jsdelivr.net/npm/qrcode/build/qrcode.min.js"></script>
</head>
<body>
    <h1>QR Code Generator</h1>
    <div>
        <label for="inputText">Enter text or URL:</label>
        <input type="text" id="inputText" placeholder="Enter text or URL">
        <button onclick="generateQRCode()">Generate QR Code</button>
    </div>
    <div id="qrCode"></div>

    <script>
        function generateQRCode() {
            var text = document.getElementById('inputText').value;
            var qrCodeContainer = document.getElementById('qrCode');
            
            // Clear previous QR Code if any
            qrCodeContainer.innerHTML = '';

            if (text) {
                QRCode.toCanvas(qrCodeContainer, text, function (error) {
                    if (error) console.error(error);
                    console.log('QR Code generated!');
                });
            } else {
                alert('Please enter some text or a URL!');
            }
        }
    </script>
</body>
</html>
