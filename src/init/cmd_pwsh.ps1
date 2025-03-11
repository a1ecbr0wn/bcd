function bcd {
    $result = (bookmark-cd $args 2>&1)
    if ($null -ne $result) {
        $resultString = $result.ToString()
        if ($resultString.StartsWith("cd ")) {
            $targetPath = $resultString.Substring(3).Trim()
            Set-Location $targetPath
        } else {
            Write-Output $result
        }
    }
}
